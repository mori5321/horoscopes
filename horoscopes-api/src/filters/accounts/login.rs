use serde::{Deserialize, Serialize};
use std::sync::Arc;
use warp::Filter;

use crate::adapters::infrastructure::providers::{
    access_token_provider::AccessTokenProviderImpl,
    time_provider::UTCTimeProvider,
};
use crate::adapters::infrastructure::{
    repositories::on_memory::account_repository::AccountRepositoryOnMemory,
    services::account_service::AccountServiceImpl,
};
use crate::filters::{errors::from_usecase_error, with_usecase};
use crate::usecases::accounts::login_usecase::{self, LogInUsecase};
use crate::usecases::Usecase;

pub fn filter(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
       + Clone {
    let account_repository =
        Arc::new(AccountRepositoryOnMemory::new());
    let account_service =
        Arc::new(AccountServiceImpl::new(account_repository.clone()));
    let time_provider = Arc::new(UTCTimeProvider::new());
    let access_token_provider =
        Arc::new(AccessTokenProviderImpl::new());
    
    let deps = login_usecase::Deps::new(
        account_repository.clone(),
        account_service.clone(),
        time_provider.clone(),
        access_token_provider.clone(),
    );
    
    let usecase = LogInUsecase::new(deps);

    warp::path("login")
        .and(warp::post())
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_usecase(usecase))
        .and_then(handler)
}

async fn handler(
    body: RequestBody,
    usecase: LogInUsecase,
) -> Result<impl warp::Reply, warp::Rejection> {
    let input = login_usecase::Input {
        email: body.login.email,
        password: body.login.password,
    };
    let result = usecase.run(input);

    match result {
        Ok(output) => {
            // TODO: UserIDとRefreshTokenを返す必要がある。
            let token_res_body = TokenResponseBody {
                access_token: output.access_token,
            };

            let res_body = ResponseBody {
                data: token_res_body,
            };
            Ok(warp::reply::json(&res_body)).map(|rep| {
                warp::reply::with_status(
                    rep,
                    warp::http::StatusCode::OK,
                )
            })
        }
        Err(err) => {
            let app_error = from_usecase_error(err);
            Err(warp::reject::custom(app_error))
        }
    }
}

#[derive(Serialize, Deserialize)]
struct RequestBody {
    login: Login,
}

#[derive(Serialize, Deserialize)]
struct Login {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct ResponseBody {
    data: TokenResponseBody,
}

#[derive(Serialize, Deserialize)]
struct TokenResponseBody {
    access_token: String,
}
