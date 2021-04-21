mod accounts;
mod oauth2;
mod todos;

mod errors;

use warp::Filter;
use std::convert::Infallible;
use crate::usecases::Usecase;

pub fn filters(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let api_prefix = warp::path("api");
    api_prefix
        .and(accounts::filters("accounts".to_string()))
        .or(oauth2::filters("oauth2".to_string()))
        .or(todos::filters("todos".to_string()))
}

fn with_usecase<U, Input, Output, Deps>(usecase: U)
    -> impl Filter<Extract = (U, ), Error = Infallible> + Clone
    where U: Usecase<Input, Output, Deps> + Clone + Send
{
    warp::any().map(move || usecase.clone())
}

