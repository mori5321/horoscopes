use crate::handlers::accounts;
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
        .and_then(accounts::login)
}

fn signup() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("signup")
        .and(warp::post())
        .and_then(accounts::signup)
}
