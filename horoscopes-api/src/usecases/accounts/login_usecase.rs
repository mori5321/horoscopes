use crate::domain::entities::account::{self, Account, Login};
use crate::domain::repositories::AccountRepository;
use crate::domain::services::account_service::AccountService;
use crate::usecases::common::ports::providers::IDProvider;
use crate::usecases::{
    common::errors::{BusinessError, UsecaseError, UsecaseErrorType},
    Usecase,
};

use chrono::Duration;
use hmac::{Hmac, NewMac};
use jwt::{AlgorithmType, Header, SignWithKey, Token};
use sha2::Sha256;
use std::collections::BTreeMap;
use std::sync::Arc;

use crate::usecases::common::ports::providers::TimeProvider;

#[derive(Clone)]
pub struct Deps {
    account_repository: Arc<dyn AccountRepository>,
    time_provider: Arc<dyn TimeProvider>,
}

impl Deps {
    pub fn new(
        account_repository: Arc<dyn AccountRepository>,
        time_provider: Arc<dyn TimeProvider>,
    ) -> Self {
        Self {
            account_repository,
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
            Some(_account) => {
                // TODO: LoginとAccountをVerifyする。
                //
                let key: Hmac<Sha256> =
                    Hmac::new_varkey(b"horoscopes-secret").unwrap();
                let header: Header = Default::default();

                let now = self.deps.time_provider.now();
                let offset = Duration::hours(1);
                let expires_at = now + offset;
                let expires_at_ts =
                    expires_at.timestamp().to_string();

                let mut claims = BTreeMap::new();
                claims.insert("user_id", "xxxxxxxx0001");
                claims.insert("expires_at", expires_at_ts.as_str());
                // Memo
                // - SecretKeyに何を使うのか
                // - Tokenの型はどんなものなのか
                //      - BtreeMapである必要性なさそう。
                //      - UserID, ExpiresAt, OrganizationID(?)

                let access_token: Token<
                    Header,
                    BTreeMap<&str, &str>,
                    _,
                > = Token::new(header, claims);
                let signed_access_token =
                    access_token.sign_with_key(&key).unwrap();

                // TODO: RefreshTokenを生成する
                // TODO: Refresh Tokenを保存する
                // TODO: Refresh Tokenを返す
                return Ok(Output {
                    access_token: signed_access_token.into(),
                });
            }
        }
    }
}
