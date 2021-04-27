use warp::Filter;

pub fn filter() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("signup")
        .and(warp::path::end())
        .and(warp::post())
        .and_then(handler)
}


pub async fn handler() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&"signup"))
        .map(|rep| warp::reply::with_status(rep, warp::http::StatusCode::OK))
}
