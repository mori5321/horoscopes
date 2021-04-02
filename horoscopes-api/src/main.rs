use horoscopes_api::filters;

#[tokio::main()]
async fn main() {
    let port: u16 = 3030;
    println!("Server is running on port: {}", port);

    warp::serve(filters::filters())
        .run(([127, 0, 0, 1], port))
        .await;
}

// mod filters {
//     use horoscopes_api::handlers::respond_with_json;
//     use warp::Filter;
//
//     pub fn filters() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//         let api_prefix = warp::path("api");
//         let end = warp::path::end().map(|| "Not Found...");
//         api_prefix.and(todos().or(end))
//     }
//
//     pub fn todos() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//         let todo_prefix = warp::path!("todos" / ..);
//         todo_prefix.and(
//             warp::get()
//                 .and_then(list_todos)
//                 .or(warp::post().and_then(post_todo)),
//         )
//     }
//
//     async fn list_todos() -> Result<impl warp::Reply, warp::Rejection> {
//         // Ok(warp::reply::json(""))
//         respond_with_json(Ok("list"), warp::http::StatusCode::OK)
//     }
//
//     async fn post_todo() -> Result<impl warp::Reply, warp::Rejection> {
//         respond_with_json(Ok("created"), warp::http::StatusCode::CREATED)
//     }
// }
