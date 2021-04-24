use warp::Filter;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use crate::filters::with_usecase;
use crate::usecases::todos::get_todo_usecase;
use crate::usecases::todos::get_todo_usecase::GetTodoUsecase;
use crate::adapters::infrastructure::repositories::on_memory::todo_repository;
use crate::usecases::Usecase;

// GET /todos/:id
pub fn filter(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
+ Clone {
    let deps = get_todo_usecase::Deps::new(
        Arc::new(todo_repository::TodoRepositoryOnMemory::new())
    );
    let usecase = GetTodoUsecase::new(deps);

    warp::get()
        .and(warp::path::param())
        .and(warp::path::end())
        .and(with_usecase(usecase))
        .and_then(handler)
}

async fn handler(id: String, usecase: GetTodoUsecase) -> Result<impl warp::Reply, warp::Rejection> {
    let input = get_todo_usecase::Input{ id };
    let output = usecase.run(input);

    match output.todo {
        None => {
            // TODO return JSON explains Client Error
            Ok(warp::reply::json(&"Return Custom Error for 404 Here"))
                .map(|rep| warp::reply::with_status(rep, warp::http::StatusCode::NOT_FOUND))
        } 
        Some(todo) => {
            let response = from_dto(todo);
            Ok(warp::reply::json(&response))
                .map(|rep| warp::reply::with_status(rep, warp::http::StatusCode::OK))
        } 
    }
}

#[derive(Serialize, Deserialize)]
struct Response {
    data: TodoResponse,
}

#[derive(Serialize, Deserialize)]
struct TodoResponse {
        id: String,
        title: String,
        is_done: bool,
} 

fn from_dto(todo_dto: get_todo_usecase::TodoDTO) -> Response {
    let todo = TodoResponse {
        id: todo_dto.id.clone(),
        title: todo_dto.title,
        is_done: todo_dto.is_done,
    };

    Response {
        data: todo
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use warp::reply::Reply;
    use serde_json::json;
    use assert_json_diff::assert_json_eq;
    use crate::domain::entities::todo;
    use crate::adapters::infrastructure::repositories::for_test::todo_repository::TodoRepositoryForTest;

    #[tokio::test]
    async fn handler_returns_todos_json_response() {
        let todos = vec![
            todo::Todo::new("ulid-0001".to_string(), "First Task".to_string(), false),
        ];
    
        let todo_repository = Arc::new(
            TodoRepositoryForTest::new(todos)
        );

        let deps = get_todo_usecase::Deps::new(todo_repository);
        let usecase = get_todo_usecase::GetTodoUsecase::new(deps);

        let rep = handler("ulid-0001".to_string(), usecase).await.unwrap();
        let res = rep.into_response();

        let status_code = res.status();
        let body = res.into_body();
        let bytes = hyper::body::to_bytes(body).await.unwrap();

        let json = json!({
            "data": {
                "id": "ulid-0001",
                "title": "First Task",
                "is_done": false,
            }
        });

        assert_eq!(status_code, warp::http::StatusCode::OK);
        assert_json_eq!(
            serde_json::from_slice::<serde_json::Value>(&bytes).unwrap(),
            json
        )
    }
}
