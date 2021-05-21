use std::sync::Arc;
use warp::Filter;

use crate::state::AppState;

mod create;

pub fn filters(
    prefix: String,
    app_state: Arc<AppState>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
       + Clone {
    let organizations_prefix = warp::path(prefix);
    organizations_prefix.and(create::filter(app_state))
}
