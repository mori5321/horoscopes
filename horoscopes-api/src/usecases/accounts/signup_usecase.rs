use crate::domain::entities::account::SignUp;
use crate::domain::entities::user::User;
use crate::domain::repositories::AccountRepository;
use crate::domain::services::account_service::AccountService;
use crate::usecases::common::ports::providers::{
    AccessTokenProvider, IDProvider, TimeProvider,
};
use crate::usecases::{
    common::errors::{
        from_domain_error, BusinessError, SystemError, UsecaseError,
        UsecaseErrorType,
    },
    Usecase,
};

use chrono::Duration;
use std::sync::Arc;

#[derive(Clone)]
pub struct Deps {
    account_repository: Arc<dyn AccountRepository>,
    account_service: Arc<dyn AccountService>,
    id_provider: Arc<dyn IDProvider>,
    time_provider: Arc<dyn TimeProvider>,
    access_token_provider: Arc<dyn AccessTokenProvider>,
}

impl Deps {
    pub fn new(
        account_repository: Arc<dyn AccountRepository>,
        account_service: Arc<dyn AccountService>,
        id_provider: Arc<dyn IDProvider>,
        time_provider: Arc<dyn TimeProvider>,
        access_token_provider: Arc<dyn AccessTokenProvider>,
    ) -> Self {
        Self {
            account_repository,
            account_service,
            id_provider,
            time_provider,
            access_token_provider,
        }
    }
}

pub struct Input {
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
}

// AccessTokenとか返すべき
pub struct Output {
    pub access_token: String,
}

#[derive(Clone)]
pub struct SignUpUsecase {
    deps: Deps,
}

impl Usecase<Input, Result<Output, UsecaseError>, Deps>
    for SignUpUsecase
{
    fn new(deps: Deps) -> Self {
        Self { deps }
    }

    fn run(&self, input: Input) -> Result<Output, UsecaseError> {
        let account_id = self.deps.id_provider.generate();
        let user_id = self.deps.id_provider.generate();

        let res_signup = SignUp::new(
            account_id,
            input.email,
            input.password,
            input.password_confirmation,
        );

        let signup =
            res_signup.map_err(|err| from_domain_error(err))?;

        if let Err(err) = validator::validate_signup(&signup) {
            return Err(err);
        }

        if self.deps.account_service.is_duplicated(&signup) {
            return Err(UsecaseError::new(
                UsecaseErrorType::BusinessError(
                    BusinessError::DuplicatedError,
                ),
                "Email is duplicated".to_string(),
            ));
        }

        let user = User::new(user_id);

        let account =
            self.deps.account_service.from_signup(&signup, user);
        if let Err(_err) =
            self.deps.account_repository.store(account.clone())
        {
            // TODO: RepositoryErrorほしいよね
            return Err(UsecaseError::new(
                UsecaseErrorType::SystemError(
                    SystemError::UnknownError,
                ),
                "UnknownError".to_string(),
            ));
        };

        let issued_at = self.deps.time_provider.now();
        let offset = Duration::minutes(60);
        let expires_at = issued_at + offset;

        let issued_at_ts = issued_at.timestamp() as u64;
        let expires_at_ts = expires_at.timestamp() as u64;
        let access_token = self.deps.access_token_provider.generate(
            account.user().id().value(),
            issued_at_ts,
            expires_at_ts,
        );

        Ok(Output { access_token })
    }
}

mod validator {
    use super::*;

    const PASSWORD_MIN_LENGTH: usize = 8;

    pub fn validate_signup(
        signup: &SignUp,
    ) -> Result<(), UsecaseError> {
        // TODO: 複数のErrorまとめて返す仕組みほしいよな
        if signup.password().value().len() < PASSWORD_MIN_LENGTH {
            return Err(UsecaseError::new(
                UsecaseErrorType::BusinessError(
                    BusinessError::ValidationError,
                ),
                format!(
                    "Password length must be more than {}",
                    PASSWORD_MIN_LENGTH
                ),
            ));
        }

        if signup.password() != signup.password_confirmation() {
            return Err(UsecaseError::new(
                UsecaseErrorType::BusinessError(
                    BusinessError::ValidationError,
                ),
                "Password and Password Confirmation do not match"
                    .to_string(),
            ));
        }

        // TODO: PasswordのFormatチェック
        // TODO: EmailのFormatチェック
        // TODO: PasswordとPasswordConfirmationのmatchチェック
        return Ok(());
    }
}
