mod accounts;

use warp::Filter;

pub fn filters() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let api_prefix = warp::path("api");
    api_prefix.and(accounts::filters("accounts".to_string()))
}
