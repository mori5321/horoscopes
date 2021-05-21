use serde::{Deserialize, Serialize};
use std::sync::Arc;
use warp::Filter;

use crate::adapters::infrastructure::repositories::persistence::{
    account_repository::AccountRepositoryImpl,
};
use crate::adapters::infrastructure::services::account_service::AccountServiceImpl;
use crate::usecases::accounts::signup_usecase::{
    self, SignUpUsecase,
};

use crate::adapters::infrastructure::providers::id::ulid_provider;
use crate::filters::errors::from_usecase_error;
use crate::filters::with_usecase;
use crate::usecases::Usecase;

use crate::state::AppState;

pub fn filter(
    app_state: Arc<AppState>
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
       + Clone {
    let account_repository =
        Arc::new(AccountRepositoryImpl::new(app_state.pool()));

    let deps = signup_usecase::Deps::new(
        account_repository.clone(),
        Arc::new(AccountServiceImpl::new(account_repository.clone())),
        Arc::new(ulid_provider::ULIDProvider::new()),
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
        Ok(_output) => Ok(warp::reply::json(&"Success")).map(|rep| {
            warp::reply::with_status(
                rep,
                warp::http::StatusCode::CREATED,
            )
        }),
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
