use warp::Filter;
use serde::{Serialize, Deserialize};
use std::sync::Arc;

use crate::filters::with_usecase;
use crate::adapters::infrastructure::repositories::todo_repository::TodoRepositoryOnMemory;
use crate::usecases::Usecase;
use crate::usecases::create_todo_usecase;
use crate::usecases::create_todo_usecase::CreateTodoUsecase;


pub fn filter(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
+ Clone {
    let deps = create_todo_usecase::Deps::new(
        Arc::new(TodoRepositoryOnMemory::new())
    );
    let usecase = CreateTodoUsecase::new(deps);

    warp::post()
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and(with_usecase(usecase))
        .and_then(handler)
}

async fn handler(body: RequestBody, usecase: CreateTodoUsecase) -> Result<impl warp::Reply, warp::Rejection> {
    format!("Body: {}", body.todo.title);
    
    let new_todo_dto = create_todo_usecase::NewTodoDTO {
        title: body.todo.title
    };
    let input = create_todo_usecase::Input { new_todo_dto };
    
    let output = usecase.run(input);
    
    // このHandling微妙よね。
    // output.success ってのがそもそも微妙な気がする
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
