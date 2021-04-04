mod accounts;
mod oauth2;

use warp::Filter;

pub fn filters() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let api_prefix = warp::path("api");
    api_prefix
        .and(accounts::filters("accounts".to_string()))
        .or(oauth2::filters("oauth2".to_string()))
}
