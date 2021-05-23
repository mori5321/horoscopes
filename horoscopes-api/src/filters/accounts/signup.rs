use serde::{Deserialize, Serialize};
use std::sync::Arc;
use warp::Filter;

use crate::adapters::infrastructure::repositories::persistence::account_repository::AccountRepositoryImpl;
use crate::adapters::infrastructure::services::account_service::AccountServiceImpl;
use crate::adapters::infrastructure::providers::time_provider::UTCTimeProvider;
use crate::adapters::infrastructure::providers::{
    id::ulid_provider,
    access_token_provider::AccessTokenProviderImpl,
};

use crate::filters::errors::from_usecase_error;
use crate::filters::with_usecase;
use crate::usecases::Usecase;
use crate::usecases::accounts::signup_usecase::{
    self, SignUpUsecase,
};

use crate::state::AppState;

pub fn filter(
    app_state: Arc<AppState>
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
       + Clone {
    let account_repository =
        Arc::new(AccountRepositoryImpl::new(app_state.pool()));
    let account_service = Arc::new(AccountServiceImpl::new(account_repository.clone()));
    let ulid_provider = Arc::new(ulid_provider::ULIDProvider::new());
    let utc_time_provider = Arc::new(UTCTimeProvider::new());
    let access_token_provider =
        Arc::new(AccessTokenProviderImpl::new());

    let deps = signup_usecase::Deps::new(
        account_repository.clone(),
        account_service,
        ulid_provider,
        utc_time_provider,
        access_token_provider,
    );

    let usecase = SignUpUsecase::new(deps);

    warp::path("signup")
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and(with_usecase(usecase))
        .and_then(handler)
}

async fn handler(
    body: RequestBody,
    usecase: SignUpUsecase,
) -> Result<impl warp::Reply, warp::Rejection> {
    let input = signup_usecase::Input {
        email: body.signup.email,
        password: body.signup.password,
        password_confirmation: body.signup.password_confirmation,
    };

    let result = usecase.run(input);


    match result {
        Ok(output) => {
            let token_res_body = TokenResponseBody {
                access_token: output.access_token,
            };
            
            let res_body = ResponseBody {
                data: token_res_body,
            };

            Ok(warp::reply::json(&res_body)).map(|rep| {
                warp::reply::with_status(
                    rep,
                    warp::http::StatusCode::CREATED,
                )
            })
        },
        Err(err) => {
            let app_error = from_usecase_error(err);
            Err(warp::reject::custom(app_error))
        }
    }
}

#[derive(Serialize, Deserialize)]
struct RequestBody {
    signup: SignUp,
}

#[derive(Serialize, Deserialize)]
struct SignUp {
    email: String,
    password: String,
    password_confirmation: String,
}

#[derive(Serialize, Deserialize)]
struct ResponseBody {
    data: TokenResponseBody
}

#[derive(Serialize, Deserialize)]
struct TokenResponseBody {
    access_token: String,
}
