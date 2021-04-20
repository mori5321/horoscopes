use warp::Filter;
use std::sync::Arc;

use crate::filters::with_usecase;
use crate::adapters::infrastructure::repositories::on_memory::todo_repository;
use crate::usecases::Usecase;
use crate::usecases::delete_todo_usecase;
use crate::usecases::delete_todo_usecase::DeleteTodoUsecase;

pub fn filter(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
+ Clone {
    let deps = delete_todo_usecase::Deps::new(
        Arc::new(todo_repository::TodoRepositoryOnMemory::new())
    );
    let usecase = DeleteTodoUsecase::new(deps);

    warp::path::param()
        .and(warp::path::end())
        .and(warp::delete())
        .and(with_usecase(usecase))
        .and_then(handler) 
}

async fn handler(id: String, usecase: DeleteTodoUsecase) -> Result<impl warp::Reply, warp::Rejection> {
    let input = delete_todo_usecase::Input { id };
    let output = usecase.run(input);

    match output.success {
        true => {
            Ok(warp::reply::json(&"Success"))
                .map(|rep| warp::reply::with_status(rep, warp::http::StatusCode::OK))
        },
        false => {
            Ok(warp::reply::json(&"Failed"))
                .map(|rep| warp::reply::with_status(rep, warp::http::StatusCode::NOT_FOUND))
        }
    }
}
