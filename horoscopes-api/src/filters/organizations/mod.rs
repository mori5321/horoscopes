use std::sync::Arc;
use warp::Filter;

use crate::state::AppState;

mod create;
mod diagnoses;

pub fn filters(
    prefix: String,
    app_state: Arc<AppState>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
       + Clone {
    let organizations_prefix = warp::path(prefix);
    organizations_prefix.and(
        create::filter(app_state.clone()).or(diagnoses::filters(
            "diagnoses".to_string(),
            app_state,
        )),
    )
}

// fn child_filters(
//     app_state: Arc<AppState>,
// ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
//        + Clone {
//     warp::path::param::<String>().and(warp::path::end()).map(
//         |organization_id: String| {
//             diagnoses::filters(
//                 "diagnoses".to_string(),
//                 app_state.clone(),
//             )
//         },
//     )
// }
