use crate::handlers::respond_with_json;

pub async fn login() -> Result<impl warp::Reply, warp::Rejection> {
    respond_with_json(Ok("login"), warp::http::StatusCode::OK)
}

pub async fn signup() -> Result<impl warp::Reply, warp::Rejection> {
    respond_with_json(Ok("signup"), warp::http::StatusCode::OK)
}
