use warp::Filter;
use serde::{Serialize, Deserialize};
use std::sync::Arc;

use crate::filters::with_usecase;
use crate::adapters::infrastructure::repositories::on_memory::todo_repository;
use crate::usecases::Usecase;
use crate::usecases::update_todo_usecase;
use crate::usecases::update_todo_usecase::UpdateTodoUsecase;

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
        Err(msg) => {
            Ok(warp::reply::json(&msg))
                .map(|rep| warp::reply::with_status(rep, warp::http::StatusCode::BAD_REQUEST))
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
