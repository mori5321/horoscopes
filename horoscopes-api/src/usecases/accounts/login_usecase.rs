use crate::domain::entities::account::Login;
use crate::domain::repositories::AccountRepository;
use crate::domain::services::account_service::AccountService;
use crate::usecases::{
    common::errors::{BusinessError, UsecaseError, UsecaseErrorType},
    Usecase,
};

use chrono::Duration;
use hmac::{Hmac, NewMac};
use jwt::{Header, SignWithKey, Token};
use sha2::Sha256;
use std::collections::BTreeMap;
use std::sync::Arc;

use crate::usecases::common::ports::providers::TimeProvider;

#[derive(Clone)]
pub struct Deps {
    account_repository: Arc<dyn AccountRepository>,
    account_service: Arc<dyn AccountService>,
    time_provider: Arc<dyn TimeProvider>,
}

impl Deps {
    pub fn new(
        account_repository: Arc<dyn AccountRepository>,
        account_service: Arc<dyn AccountService>,
        time_provider: Arc<dyn TimeProvider>,
    ) -> Self {
        Self {
            account_repository,
            account_service,
            time_provider,
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

                let key: Hmac<Sha256> =
                    Hmac::new_varkey(b"horoscopes-secret").unwrap();
                let header: Header = Default::default();

                let now = self.deps.time_provider.now();
                let offset = Duration::hours(1);
                let expires_at = now + offset;
                let expires_at_ts =
                    expires_at.timestamp().to_string();

                let mut claims = BTreeMap::new();
                claims.insert("user_id", account.user_id().value());
                claims.insert("expires_at", expires_at_ts);
                // Memo
                // - SecretKeyに何を使うのか
                // - Tokenの型はどんなものなのか
                //      - BtreeMapである必要性なさそう。独自型で共通化したいっちゃしたい。
                //      - UserID, ExpiresAt, OrganizationID(?)

                let access_token: Token<
                    Header,
                    BTreeMap<&str, String>,
                    _,
                > = Token::new(header, claims);
                let signed_access_token =
                    access_token.sign_with_key(&key).unwrap();

                // Refresh Tokenの保存・生成・返却
                return Ok(Output {
                    access_token: signed_access_token.into(),
                });
            }
        }
    }
}
