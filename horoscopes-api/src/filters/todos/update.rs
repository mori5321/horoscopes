use warp::Filter;

pub fn filter(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
+ Clone {
    warp::path::param()
        .and(warp::path::end())
        .and(warp::patch())
        .and_then(handler)
}

async fn handler(id: String) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&format!("update todo, {}", id)))
        .map(|rep| warp::reply::with_status(rep, warp::http::StatusCode::OK))
}
