use warp::Filter;

pub fn filter() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("login")
        .and(warp::post())
        .and(warp::path::end())
        .and_then(handler)
}

async fn handler() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&"login"))
        .map(|rep| warp::reply::with_status(rep, warp::http::StatusCode::OK))
}
