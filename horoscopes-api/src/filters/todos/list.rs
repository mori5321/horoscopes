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

async fn handler(usecase: list_todos_usecase::ListTodosUsecase)
    -> Result<impl warp::Reply, warp::Rejection> {
    // -> Result<warp::reply::WithStatus<warp::reply::Json>, warp::Rejection> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use warp::reply::Reply;
    use serde_json::json;
    use assert_json_diff::assert_json_eq;

    // TODO: 結合テストとして 別ディレクトリで書く
    // #[tokio::test]
    // async fn filter_returns_200_ok() {
    //     let f = filter();
    //     let value = warp::test::request()
    //         .path("/")
    //         .reply(&f)
    //         .await;
    // 
    //     let json = json!({
    //             "data": [
    //                 {
    //                     "id": "a",
    //                     "title": "hello",
    //                     "is_done": false,
    //                 },
    //                 {
    //                     "id": "b",
    //                     "title": "world",
    //                     "is_done": false,
    //                 },
    //                 {
    //                     "id": "c",
    //                     "title": "Let's Sing!",
    //                     "is_done": false,
    //                 }
    //             ]
    //         });
    // 
    //     assert_eq!(value.status(), 200);
    //     assert_eq!(
    //         serde_json::from_slice::<serde_json::Value>(value.body()).unwrap(),
    //         json
    //     );
    // }

    #[tokio::test]
    async fn handler_returns_3_todos() {
        let todo_repository = Arc::new(
            todo_repository::TodoRepositoryOnMemory::new()
        );
        let deps = list_todos_usecase::Deps::new(todo_repository);
        let usecase = list_todos_usecase::ListTodosUsecase::new(deps);

        let rep = handler(usecase).await.unwrap();
        let res = rep.into_response();
       
        let status_code = res.status();
        let body = res.into_body();
        let bytes = hyper::body::to_bytes(body).await.unwrap();
        
        let json = json!({
                "data": [
                    {
                        "id": "ulid-00000001",
                        "title": "hello",
                        "is_done": false,
                    },
                    {
                        "id": "ulid-00000002",
                        "title": "world",
                        "is_done": false,
                    },
                    {
                        "id": "ulid-00000003",
                        "title": "Let's Sing!",
                        "is_done": false,
                    }
                ]
            });
        
        assert_eq!(status_code, 200);

        // TODO: diffが見えづらいのでこれつかう
        // assert_json_diff
        // https://qiita.com/yagince/items/b60e4e59bbca7f85bcb4#test-2
        // https://qiita.com/yagince/items/b60e4e59bbca7f85bcb4#test-2
        assert_json_eq!(
            serde_json::from_slice::<serde_json::Value>(&bytes).unwrap(),
            json
        );
    }
}
