use warp::Filter;
use serde::{Serialize, Deserialize};
use std::sync::Arc;

use crate::adapters::infrastructure::repositories::on_memory::{
    account_repository::AccountRepositoryOnMemory,
    user_repository::UserRepositoryOnMemory,
};
use crate::adapters::infrastructure::services::account_service::AccountServiceImpl;
use crate::usecases::accounts::signup_usecase::{self, SignUpUsecase};

use crate::usecases::Usecase;
use crate::filters::with_usecase;
use crate::filters::errors::from_usecase_error;
use crate::adapters::infrastructure::providers::id::ulid_provider;

pub fn filter() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let account_repository = Arc::new(AccountRepositoryOnMemory::new());
    let user_repository = Arc::new(UserRepositoryOnMemory::new());

    let deps = signup_usecase::Deps::new(
        account_repository.clone(),
        Arc::new(AccountServiceImpl::new(account_repository.clone())),
        user_repository.clone(),
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


async fn handler(body: RequestBody, usecase: SignUpUsecase) -> Result<impl warp::Reply, warp::Rejection> {
    let input = signup_usecase::Input {
        email: body.signup.email,
        password: body.signup.password,
        password_confirmation: body.signup.password_confirmation,
    };
    
    let result = usecase.run(input);

    match result {
        Ok(_output) => {
            Ok(warp::reply::json(&"Success"))
                .map(|rep| warp::reply::with_status(rep, warp::http::StatusCode::CREATED))
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
    password_confirmation: String
}

