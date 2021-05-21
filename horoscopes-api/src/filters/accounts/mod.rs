use crate::state::AppState;
use std::sync::Arc;
use warp::Filter;

mod login;
mod signup;

pub fn filters(
    prefix: String,
    app_state: Arc<AppState>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
       + Clone {
    let accounts_prefix = warp::path(prefix);
    accounts_prefix.and(
        login::filter(app_state.clone())
            .or(signup::filter(app_state.clone())),
    )
}
