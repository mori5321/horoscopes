use crate::state::AppState;
use std::sync::Arc;
use warp::Filter;

mod list;

pub fn filters(
    prefix: String,
    app_state: Arc<AppState>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
       + Clone {
    let diagnoses_prefix = warp::path(prefix);
    diagnoses_prefix.and(list::filter(app_state.clone()))
}
