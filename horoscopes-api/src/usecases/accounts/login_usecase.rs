use crate::domain::entities::account::Login;
use crate::domain::repositories::AccountRepository;
use crate::domain::services::account_service::AccountService;
use crate::usecases::{
    common::errors::{BusinessError, UsecaseError, UsecaseErrorType},
    Usecase,
};

use chrono::Duration;
use std::sync::Arc;

use crate::usecases::common::ports::providers::{
    AccessTokenProvider, TimeProvider,
};

#[derive(Clone)]
pub struct Deps {
    account_repository: Arc<dyn AccountRepository>,
    account_service: Arc<dyn AccountService>,
    time_provider: Arc<dyn TimeProvider>,
    access_token_provider: Arc<dyn AccessTokenProvider>,
}

impl Deps {
    pub fn new(
        account_repository: Arc<dyn AccountRepository>,
        account_service: Arc<dyn AccountService>,
        time_provider: Arc<dyn TimeProvider>,
        access_token_provider: Arc<dyn AccessTokenProvider>,
    ) -> Self {
        Self {
            account_repository,
            account_service,
            time_provider,
            access_token_provider,
        }
    }
}

pub struct Input {
    pub email: String,
    pub password: String,
}

pub struct Output {
    pub access_token: String,
}

#[derive(Clone)]
pub struct LogInUsecase {
    deps: Deps,
}

impl Usecase<Input, Result<Output, UsecaseError>, Deps>
    for LogInUsecase
{
    fn new(deps: Deps) -> Self {
        Self { deps }
    }

    fn run(&self, input: Input) -> Result<Output, UsecaseError> {
        let login = Login::new(input.email, input.password);

        let res_account =
            self.deps.account_repository.find_by_email(login.email());

        match res_account {
            None => {
                // TODO: Repositoryエラーを返す。
                return Err(UsecaseError::new(
                    UsecaseErrorType::BusinessError(
                        BusinessError::NotFoundError,
                    ),
                    "Account is not found".to_string(),
                ));
            }
            Some(account) => {
                if !self.deps.account_service.verify(&login, &account)
                {
                    return Err(UsecaseError::new(
                        UsecaseErrorType::BusinessError(
                            BusinessError::ValidationError,
                        ),
                        "Password is invalid".to_string(),
                    ));
                }

                let issued_at = self.deps.time_provider.now();
                let offset = Duration::minutes(1);
                let expires_at = issued_at + offset;

                let issued_at_ts = issued_at.timestamp() as u64;
                let expires_at_ts = expires_at.timestamp() as u64;
                let access_token =
                    self.deps.access_token_provider.generate(
                        account.user_id().value(),
                        issued_at_ts,
                        expires_at_ts,
                    );

                return Ok(Output { access_token });
            }
        }
    }
}
