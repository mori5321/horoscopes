use warp::Filter;
use serde::{Serialize, Deserialize};
use std::sync::Arc;

use crate::filters::with_usecase;
use crate::filters::errors::from_usecase_error;
use crate::adapters::infrastructure::repositories::on_memory::todo_repository;
use crate::usecases::Usecase;
use crate::usecases::todos::update_todo_usecase;
use crate::usecases::todos::update_todo_usecase::UpdateTodoUsecase;

pub fn filter(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
+ Clone {
    let deps = update_todo_usecase::Deps::new(
        Arc::new(todo_repository::TodoRepositoryOnMemory::new())
    );
    let usecase = UpdateTodoUsecase::new(deps);

    warp::path::param()
        .and(warp::path::end())
        .and(warp::patch())
        .and(warp::body::json())
        .and(with_usecase(usecase))
        .and_then(handler)
}

async fn handler(
    id: String,
    body: RequestBody,
    usecase: UpdateTodoUsecase
) -> Result<impl warp::Reply, warp::Rejection> {
    let update_todo_dto = update_todo_usecase::UpdateTodoDTO {
        id,
        title: body.todo.title,
        is_done: body.todo.is_done
    };
    let input = update_todo_usecase::Input { update_todo_dto };
    let result = usecase.run(input);

    match result {
        Ok(_output) => {
            Ok(warp::reply::json(&"Success"))
                .map(|rep| warp::reply::with_status(rep, warp::http::StatusCode::NO_CONTENT))
        },
        Err(err) => {
            let app_error = from_usecase_error(err);
            Err(warp::reject::custom(app_error))
        }
    }
}


#[derive(Serialize, Deserialize)]
struct RequestBody {
    todo: UpdateTodo
}

#[derive(Serialize, Deserialize)]
struct UpdateTodo {
    is_done: bool,
    title: String,
}


#[cfg(test)]
mod tests {
    use super::*;
    use warp::reply::Reply;
    use crate::adapters::infrastructure::repositories::for_test::todo_repository::TodoRepositoryForTest;
    use crate::filters::errors::AppErrorType;
    use crate::domain::entities::todo;
    use crate::domain::repositories::TodoRepository;

    fn init_usecase() -> (update_todo_usecase::UpdateTodoUsecase, Arc<TodoRepositoryForTest>) {
        let todo_repository = Arc::new(
            TodoRepositoryForTest::new(vec![
                todo::Todo::new(
                    "ulid-0001".to_string(),
                    "Fly me to the moon".to_string(),
                    false,
                )
            ])
        );
        let deps = update_todo_usecase::Deps::new(
            todo_repository.clone()
        );
        let usecase = update_todo_usecase::UpdateTodoUsecase::new(deps);

        return (usecase, todo_repository)
    }

    #[tokio::test]
    async fn handler_updates_todo_properly() {
        let (usecase, todo_repository) = init_usecase();

        let update_todo = UpdateTodo {
            is_done: true,
            title: "What kind of fool I am".to_string(),
        };

        let req_body = RequestBody {
            todo: update_todo
        };
        let rep = handler("ulid-0001".to_string(), req_body, usecase).await.unwrap();
        let res = rep.into_response();

        let status_code = res.status();

        assert_eq!(status_code, warp::http::StatusCode::NO_CONTENT);

        let new_todo = todo_repository.find(
            todo::ID::from_str("ulid-0001")
        ).unwrap();
        
        assert_eq!(new_todo.title().value(), "What kind of fool I am".to_string());
        assert_eq!(new_todo.is_done().value(), true);
    }

    #[tokio::test]
    async fn handler_returns_not_found_error_when_invalid_id_passed() {
        let (usecase, todo_repository) = init_usecase();

        let update_todo = UpdateTodo {
            is_done: true,
            title: "What kind of fool I am".to_string(),
        };

        let req_body = RequestBody {
            todo: update_todo
        };
        
        if let Err(rejection) = handler("invalid_id".to_string(), req_body, usecase).await {
            let err = rejection.find::<crate::filters::errors::AppError>().unwrap();
            assert_eq!(err.err_type, AppErrorType::NotFound);

            let todos = todo_repository.list();
            assert_eq!(todos.len(), 1);

            return;
        };
        assert!(false);
    }
}
