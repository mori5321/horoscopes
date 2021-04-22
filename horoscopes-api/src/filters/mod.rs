mod accounts;
mod oauth2;
mod todos;

mod errors;

use std::convert::Infallible;
use warp::Filter;
use warp::http::StatusCode;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::usecases::Usecase;
use crate::filters::errors::AppErrorType;

pub fn filters(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let api_prefix = warp::path("api");
    api_prefix
        .and(accounts::filters("accounts".to_string()))
        .or(oauth2::filters("oauth2".to_string()))
        .or(todos::filters("todos".to_string()))
        .recover(handle_rejection)
}

struct ErrorResponse {
    code: StatusCode,
    message: String
}

impl Serialize for ErrorResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ErrorResponse", 2)?;
        s.serialize_field("code", &self.code.as_u16())?;
        s.serialize_field("message", &self.message)?;
        s.end()
    }
}

async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    let resp: ErrorResponse;

    if let Some(e) = err.find::<crate::filters::errors::AppError>() {
        resp = match e.err_type {
            AppErrorType::NotFound => {
                ErrorResponse {
                    code: StatusCode::NOT_FOUND,
                    message: e.message.clone(),
                }
            },
            AppErrorType::Internal => {
                ErrorResponse {
                    code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: e.message.clone(),
                }
            },
            AppErrorType::UnprocessableEntity => {
                ErrorResponse {
                    code: StatusCode::UNPROCESSABLE_ENTITY,
                    message: e.message.clone(),
                }
            },
            AppErrorType::BadRequest => {
                ErrorResponse {
                    code: StatusCode::BAD_REQUEST,
                    message: e.message.clone(),
                }
            }
        };

        let json = warp::reply::json(&resp);
        return Ok(warp::reply::with_status(json, resp.code))
    } else {
        // TODO: warp組み込みのエラーのResponseをJSONに変換したい。
        return Err(err)
    }
}

fn with_usecase<U, Input, Output, Deps>(usecase: U)
    -> impl Filter<Extract = (U, ), Error = Infallible> + Clone
    where U: Usecase<Input, Output, Deps> + Clone + Send
{
    warp::any().map(move || usecase.clone())
}

