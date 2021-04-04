pub mod accounts;
pub mod oauth2;

use crate::errors::AppError;
use serde::Serialize;

pub fn respond_with_json<T: Serialize>(
    result: Result<T, AppError>,
    status: warp::http::StatusCode,
) -> Result<impl warp::Reply, warp::Rejection> {
    match result {
        Ok(response) => Ok(warp::reply::with_status(
            warp::reply::json(&response),
            status,
        )),
        Err(err) => Err(warp::reject::custom(err)),
    }
}
