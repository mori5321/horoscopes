use crate::state::AppState;
use std::sync::Arc;
use warp::Filter;

mod login;
mod signup;

pub fn filters(
    path: String,
    app_state: Arc<AppState>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
       + Clone {
    let accounts_path = warp::path(path);
    accounts_path.and(
        login::filter(app_state.clone())
            .or(signup::filter(app_state.clone())),
    )
}
