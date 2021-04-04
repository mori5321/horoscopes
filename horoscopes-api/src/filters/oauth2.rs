use crate::handlers::oauth2;
use warp::Filter;

pub fn filters(
    prefix: String,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let oauth2_prefix = warp::path(prefix);
    oauth2_prefix.and(auth().or(callback()))
}

fn auth() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("auth").and(warp::get()).and_then(oauth2::auth)
}

fn callback() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("callback")
        .and(warp::get())
        .and(warp::query::<oauth2::CallbackQueries>())
        .and_then(oauth2::callback)
}
