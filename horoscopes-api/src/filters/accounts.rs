use warp::Filter;

pub fn filters(
    prefix: String,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let accounts_prefix = warp::path(prefix);
    accounts_prefix.and(login().or(signup()))
}

fn login() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("login")
        .and(warp::post())
        .and_then(login_handler)
}

fn signup() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("signup")
        .and(warp::post())
        .and_then(signup_handler)
}

pub async fn login_handler() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&"login"))
        .map(|rep| warp::reply::with_status(rep, warp::http::StatusCode::OK))
}

pub async fn signup_handler() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&"signup"))
        .map(|rep| warp::reply::with_status(rep, warp::http::StatusCode::OK))
}
