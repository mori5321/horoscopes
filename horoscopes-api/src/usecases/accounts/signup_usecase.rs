use crate::domain::entities::account::SignUp;
use crate::domain::entities::user::User;
use crate::domain::repositories::{
    AccountRepository, UserRepository,
};
use crate::domain::services::account_service::AccountService;
use crate::usecases::common::ports::providers::IDProvider;
use crate::usecases::{
    common::errors::{BusinessError, UsecaseError, UsecaseErrorType},
    Usecase,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct Deps {
    account_repository: Arc<dyn AccountRepository>,
    account_service: Arc<dyn AccountService>,
    user_repository: Arc<dyn UserRepository>,
    id_provider: Arc<dyn IDProvider>,
}

impl Deps {
    pub fn new(
        account_repository: Arc<dyn AccountRepository>,
        account_service: Arc<dyn AccountService>,
        user_repository: Arc<dyn UserRepository>,
        id_provider: Arc<dyn IDProvider>,
    ) -> Self {
        Self {
            account_repository,
            account_service,
            user_repository,
            id_provider,
        }
    }
}

pub struct Input {
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
}

// AccessTokenとか返すべき
pub struct Output {}

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

        let signup = SignUp::new(
            account_id,
            input.email,
            input.password,
            input.password_confirmation,
        );

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

        // 空のUser作成する。
        let empty_user = User::new(user_id, "".to_string());
        self.deps
            .user_repository
            .store(empty_user.clone())
            .expect("Failed to save user");

        let account = self
            .deps
            .account_service
            .from_signup(&signup, empty_user.id());
        self.deps.account_repository.store(account).unwrap();

        Ok(Output {})
    }
}

mod validator {
    use super::*;

    const PASSWORD_MIN_LENGTH: usize = 8;

    pub fn validate_signup(
        signup: &SignUp,
    ) -> Result<(), UsecaseError> {
        // WANT: Errorまとめて返す仕組みほしいよな
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
