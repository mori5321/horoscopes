use std::sync::Arc;
use warp::Filter;

use crate::state::AppState;

mod create;
mod diagnoses;

pub fn filters(
    path: String,
    app_state: Arc<AppState>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
       + Clone {
    let organizations_path = warp::path(path);
    organizations_path.and(
        create::filter(app_state.clone())
            .or(child_filters(app_state.clone())),
    )
}

fn child_filters(
    app_state: Arc<AppState>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
       + Clone {
    diagnoses::filters("diagnoses".to_string(), app_state)
}
