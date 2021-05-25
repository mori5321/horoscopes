use crate::state::AppState;
use std::sync::Arc;
use warp::Filter;

mod list;

pub fn filters(
    prefix: String,
    app_state: Arc<AppState>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
       + Clone {
    // 及第点として prefixを末端のFilterに渡してそこで処理すればよいのでは...?
    // あるいはRoutingレイヤーは1箇所にまとめるべきだったか...?
    let diagnoses_prefix = warp::path(prefix);
    diagnoses_prefix.and(list::filter(app_state.clone()))
}

// Memo: How to define Nested Filter.
// https://github.com/seanmonstar/warp/issues/744
// https://github.com/seanmonstar/warp/pull/693/files
