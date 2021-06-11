mod accounts;
mod oauth2;
mod todos;
mod organizations;

mod errors;

use std::sync::Arc;
use std::convert::Infallible;
use warp::Filter;
use warp::http::StatusCode;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::usecases::{
    Usecase,
    common::ports::providers::AccessTokenProvider,
};
use crate::filters::errors::{AppErrorType, AppError};
use crate::adapters::infrastructure::providers::access_token_provider::AccessTokenProviderImpl;
use crate::state::AppState;

pub fn filters(
    app_state: Arc<AppState>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    // TODO: 環境変数からもらいたい。
    // TODO: cors変数自体はstaticにしてもよさそう。
    let cors = warp::cors()
        .allow_origins(vec!["http://localhost:8080"]) 
        .allow_headers(vec!["content-type", "accept"])
        .allow_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTION"]);

    accounts::filters("accounts".to_string(), app_state.clone())
        .or(oauth2::filters("oauth2".to_string()))
        .or(todos::filters("todos".to_string()))
        .or(organizations::filters("organizations".to_string(), app_state.clone()))
        .recover(handle_rejection)
        .with(cors)
}

struct ErrorResponse {
    code: StatusCode,
    message: String,
}

impl Serialize for ErrorResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s =
            serializer.serialize_struct("ErrorResponse", 2)?;
        s.serialize_field("code", &self.code.as_u16())?;
        s.serialize_field("message", &self.message)?;
        s.end()
    }
}

async fn handle_rejection(
    err: warp::Rejection,
) -> Result<impl warp::Reply, warp::Rejection> {
    let resp: ErrorResponse;

    if let Some(e) = err.find::<crate::filters::errors::AppError>() {
        resp = match e.err_type {
            AppErrorType::NotFound => ErrorResponse {
                code: StatusCode::NOT_FOUND,
                message: e.message.clone(),
            },
            AppErrorType::Internal => ErrorResponse {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                message: e.message.clone(),
            },
            AppErrorType::UnprocessableEntity => ErrorResponse {
                code: StatusCode::UNPROCESSABLE_ENTITY,
                message: e.message.clone(),
            },
            AppErrorType::BadRequest => ErrorResponse {
                code: StatusCode::BAD_REQUEST,
                message: e.message.clone(),
            },
            AppErrorType::Unauthorized => ErrorResponse {
                code: StatusCode::UNAUTHORIZED,
                message: e.message.clone(),
            }
        };

        let json = warp::reply::json(&resp);
        return Ok(warp::reply::with_status(json, resp.code));
    } else {
        // TODO: warp組み込みのエラーのResponseをJSONに変換したい。
        println!("Handle Rejection: {:?}", err);
        return Err(err);
    }
}

fn with_usecase<U, Input, Output, Deps>(
    usecase: U,
) -> impl Filter<Extract = (U,), Error = Infallible> + Clone
where
    U: Usecase<Input, Output, Deps> + Clone + Send,
{
    warp::any().map(move || usecase.clone())
}

fn with_auth(
) -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone
{
    warp::header::<String>("authorization")
        .map(|authorization: String| {
            authorization
                .trim()
                .strip_prefix("Bearer ")
                .unwrap_or("")
                .to_string()
        })
        .and_then(authorize)
}

async fn authorize(
    access_token: String,
) -> Result<String, warp::Rejection> {
    let access_token_provider = AccessTokenProviderImpl::new();
    match access_token_provider.verify(access_token) {
        Ok(user_id) => {
            Ok(user_id)
        },
        Err(text) => {
            let err = AppError::new(
                AppErrorType::Unauthorized,
                text
            );
            Err(warp::reject::custom(err))
        }
    }
}
