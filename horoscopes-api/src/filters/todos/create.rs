use serde::{Deserialize, Serialize};
use std::sync::Arc;
use warp::Filter;

use crate::adapters::infrastructure::providers::id::ulid_provider;
use crate::adapters::infrastructure::repositories::on_memory::todo_repository;
use crate::filters::errors::{from_usecase_error, AppError};
use crate::filters::with_usecase;
use crate::usecases::todos::create_todo_usecase;
use crate::usecases::todos::create_todo_usecase::CreateTodoUsecase;
use crate::usecases::Usecase;

pub fn filter(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
       + Clone {
    let deps = create_todo_usecase::Deps::new(
        Arc::new(todo_repository::TodoRepositoryOnMemory::new()),
        Arc::new(ulid_provider::ULIDProvider::new()),
    );
    let usecase = CreateTodoUsecase::new(deps);

    warp::post()
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_usecase(usecase))
        .and_then(handler)
}

async fn handler(
    body: RequestBody,
    usecase: CreateTodoUsecase,
) -> Result<impl warp::Reply, warp::Rejection> {
    let new_todo_dto = create_todo_usecase::NewTodoDTO {
        title: body.todo.title,
    };
    let input = create_todo_usecase::Input { new_todo_dto };

    let result = usecase.run(input);

    match result {
        Err(err) => {
            let app_error = from_usecase_error(err);
            Err(warp::reject::custom::<AppError>(app_error))
        }
        Ok(output) => {
            let todo_response = CreateTodoResponse { id: output.id };
            let response = Response {
                data: todo_response,
            };

            Ok(warp::reply::json(&response)).map(|rep| {
                warp::reply::with_status(
                    rep,
                    warp::http::StatusCode::CREATED,
                )
            })
        }
    }
}

#[derive(Serialize, Deserialize)]
struct RequestBody {
    todo: NewTodo,
}

#[derive(Serialize, Deserialize)]
struct NewTodo {
    title: String,
}

// TODO: これ共通型つくったほうがよさそうだな
#[derive(Serialize, Deserialize)]
struct Response {
    data: CreateTodoResponse,
}

#[derive(Serialize, Deserialize)]
struct CreateTodoResponse {
    id: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use warp::reply::Reply;
    use crate::adapters::infrastructure::repositories::for_test::todo_repository::TodoRepositoryForTest;
    use crate::filters::errors::AppErrorType;
    use crate::domain::repositories::TodoRepository;

    fn init_usecase() -> (
        create_todo_usecase::CreateTodoUsecase,
        Arc<TodoRepositoryForTest>,
    ) {
        let todo_repository =
            Arc::new(TodoRepositoryForTest::new(vec![]));
        let deps = create_todo_usecase::Deps::new(
            todo_repository.clone(),
            Arc::new(ulid_provider::ULIDProvider::new()),
        );
        let usecase =
            create_todo_usecase::CreateTodoUsecase::new(deps);

        return (usecase, todo_repository);
    }

    const ULID_LENGTH: usize = 26;

    #[tokio::test]
    async fn handler_creates_todo() {
        let (usecase, todo_repository) = init_usecase();

        let new_todo = NewTodo {
            title: "NewTodo".to_string(),
        };
        let req_body = RequestBody { todo: new_todo };
        let rep = handler(req_body, usecase).await.unwrap();
        let res = rep.into_response();

        let status_code = res.status();

        let todos = todo_repository.todos.lock().unwrap();
        let first_todo = &todos[0];

        assert_eq!(first_todo.title().value(), "NewTodo".to_string());
        assert_eq!(first_todo.is_done().value(), false);
        assert_eq!(first_todo.id().value().len(), ULID_LENGTH);

        assert_eq!(status_code, warp::http::StatusCode::CREATED);
    }

    #[tokio::test]
    async fn handler_returns_201_when_title_is_less_than_80_letters()
    {
        let (usecase, _) = init_usecase();

        let new_todo = NewTodo {
            title: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string()
        };
        assert!(&new_todo.title.len() <= &80);

        let req_body = RequestBody { todo: new_todo };
        let rep = handler(req_body, usecase).await.unwrap();
        let res = rep.into_response();

        let status_code = res.status();

        assert_eq!(status_code, warp::http::StatusCode::CREATED);
    }

    #[tokio::test]
    async fn handler_returns_unprocessable_entity_error_when_title_is_over_80_letters(
    ) {
        let (usecase, todo_repository) = init_usecase();

        let new_todo = NewTodo {
            title: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string()
        };
        assert!(&new_todo.title.len() > &80);

        let req_body = RequestBody { todo: new_todo };

        if let Err(rejection) = handler(req_body, usecase).await {
            let err = rejection
                .find::<crate::filters::errors::AppError>()
                .unwrap();
            assert_eq!(
                err.err_type,
                AppErrorType::UnprocessableEntity
            );

            let todos = todo_repository.list();
            assert_eq!(todos.len(), 0);

            return;
        }
        assert!(false)
    }
}
