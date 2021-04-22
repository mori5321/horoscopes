use warp::Filter;
use serde::{Serialize, Deserialize};
use std::sync::Arc;

use crate::filters::with_usecase;
use crate::adapters::infrastructure::repositories::on_memory::todo_repository;
use crate::usecases::Usecase;
use crate::usecases::create_todo_usecase;
use crate::usecases::create_todo_usecase::CreateTodoUsecase;


pub fn filter(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
+ Clone {
    let deps = create_todo_usecase::Deps::new(
        Arc::new(todo_repository::TodoRepositoryOnMemory::new())
    );
    let usecase = CreateTodoUsecase::new(deps);

    warp::post()
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_usecase(usecase))
        .and_then(handler)
}

async fn handler(body: RequestBody, usecase: CreateTodoUsecase) -> Result<impl warp::Reply, warp::Rejection> {
    format!("Create");
    format!("Body: {}", body.todo.title);
    
    let new_todo_dto = create_todo_usecase::NewTodoDTO {
        title: body.todo.title
    };
    let input = create_todo_usecase::Input { new_todo_dto };
    
    let output = usecase.run(input);
    
    // TODO: このError Handling微妙。
    // Errorを返したほうがよい。
    match output.success {
        true => {
            Ok(warp::reply::json(&"Success"))
                .map(|rep| warp::reply::with_status(rep, warp::http::StatusCode::CREATED))
        },
        false => {
            Ok(warp::reply::json(&"Failed"))            
                .map(|rep| warp::reply::with_status(rep, warp::http::StatusCode::BAD_REQUEST))
        }
    }
}

#[derive(Serialize, Deserialize)]
struct RequestBody {
    todo: NewTodo
}

#[derive(Serialize, Deserialize)]
struct NewTodo {
    title: String,
}


#[cfg(test)]
mod tests {
    use super::*;
    use warp::reply::Reply;
    use crate::adapters::infrastructure::repositories::for_test::todo_repository::TodoRepositoryForTest;

    fn init_usecase() -> (create_todo_usecase::CreateTodoUsecase, Arc<TodoRepositoryForTest>) {
        let todo_repository = Arc::new(
            TodoRepositoryForTest::new(vec![])
        );
        let deps = create_todo_usecase::Deps::new(todo_repository.clone());
        let usecase = create_todo_usecase::CreateTodoUsecase::new(deps);

        return (usecase, todo_repository)
    }

    #[tokio::test]
    async fn handler_creates_todo() {
        let (usecase, todo_repository) = init_usecase();

        let new_todo = NewTodo {
            title: "NewTodo".to_string()
        };
        let req_body = RequestBody {
            todo: new_todo
        };
        let rep = handler(req_body, usecase).await.unwrap();
        let res = rep.into_response();

        let status_code = res.status();


        let todos = todo_repository.todos.lock().unwrap();
        let first_todo = &todos[0];

        assert_eq!(first_todo.title().value(), "NewTodo".to_string());
        assert_eq!(first_todo.is_done().value(), false);
        assert_eq!(first_todo.id().value(), "xxxxxx".to_string());
        
        assert_eq!(status_code, 201);
    }
    
    #[tokio::test]
    async fn handler_returns_201_when_title_is_less_than_80_letters() {
        let (usecase, _) = init_usecase();

        let new_todo = NewTodo {
            title: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string()
        };
        assert!(&new_todo.title.len() <= &80);
        
        let req_body = RequestBody {
            todo: new_todo
        };
        let rep = handler(req_body, usecase).await.unwrap();
        let res = rep.into_response();

        let status_code = res.status();
        
        assert_eq!(status_code, 201);
    }

    #[tokio::test]
    async fn handler_returns_400_when_title_is_over_80_letters() {
        let (usecase, _) = init_usecase();

        let new_todo = NewTodo {
            title: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string()
        };
        assert!(&new_todo.title.len() > &80);
        
        let req_body = RequestBody {
            todo: new_todo
        };
        let rep = handler(req_body, usecase).await.unwrap();
        let res = rep.into_response();

        let status_code = res.status();
        
        assert_eq!(status_code, 400);
    }
}

