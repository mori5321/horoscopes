pub async fn login() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&"login"))
        .map(|rep| warp::reply::with_status(rep, warp::http::StatusCode::OK))
}

pub async fn signup() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&"signup"))
        .map(|rep| warp::reply::with_status(rep, warp::http::StatusCode::OK))
}
