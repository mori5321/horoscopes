use warp::Filter;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use crate::filters::with_usecase;
use crate::usecases::list_todos_usecase;
use crate::adapters::infrastructure::repositories::todo_repository;
use crate::usecases::Usecase;

pub fn filter(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let todo_repository = Arc::new(
        todo_repository::TodoRepositoryOnMemory::new()
    );
    let deps = list_todos_usecase::Deps::new(todo_repository);
    let usecase = list_todos_usecase::ListTodosUsecase::new(deps);
    
    warp::path::end()
        .and(warp::get())
        .and(with_usecase(usecase))
        .and_then(handler) 
}

async fn handler(usecase: list_todos_usecase::ListTodosUsecase) -> Result<impl warp::Reply, warp::Rejection> {
    let input = list_todos_usecase::Input{};
    let output = usecase.run(input);

    let response = from_dto(output.todos);
    
    Ok(warp::reply::json(&response))
        .map(|rep| warp::reply::with_status(rep, warp::http::StatusCode::OK))
}

#[derive(Serialize, Deserialize)]
struct Response {
    data: Vec<TodoResponse>
}

#[derive(Serialize, Deserialize)]
struct TodoResponse {
    id: String,
    title: String,
    is_done: bool,
}

fn from_dto(todos_dto: list_todos_usecase::TodosDTO) -> Response {
    let todos = todos_dto.into_iter().map(|dto| {
        TodoResponse {
            id: dto.id,
            title: dto.title,
            is_done: dto.is_done,
        }
    }).collect();
    
    Response {
        data: todos
    }
}
