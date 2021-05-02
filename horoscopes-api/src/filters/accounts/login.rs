use warp::Filter;
use serde::{Serialize, Deserialize};
use std::sync::Arc;

use crate::adapters::infrastructure::{
    repositories::on_memory::account_repository::AccountRepositoryOnMemory,
    // services::account_service::AccountServiceImpl
};
use crate::usecases::accounts::login_usecase::{self, LogInUsecase};
use crate::usecases::Usecase;
use crate::filters::{
    with_usecase,
    errors::from_usecase_error
};

pub fn filter() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let repository = Arc::new(AccountRepositoryOnMemory::new());
    let deps = login_usecase::Deps::new(
        repository.clone(),
    );
    let usecase = LogInUsecase::new(deps);

    warp::path("login")
        .and(warp::post())
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_usecase(usecase))
        .and_then(handler)
}

async fn handler(body: RequestBody, usecase: LogInUsecase) -> Result<impl warp::Reply, warp::Rejection> {
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

            let res_body = ResponseBody { data: token_res_body };
            Ok(warp::reply::json(&res_body))
                .map(|rep| warp::reply::with_status(rep, warp::http::StatusCode::OK))
        },
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
    data: TokenResponseBody 
}

#[derive(Serialize, Deserialize)]
struct TokenResponseBody {
    access_token: String,
}
