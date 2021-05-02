use crate::domain::entities::account::{self, Account, Login};
use crate::domain::repositories::AccountRepository;
use crate::domain::services::account_service::AccountService;
use crate::usecases::common::ports::providers::IDProvider;
use crate::usecases::{
    common::errors::{BusinessError, UsecaseError, UsecaseErrorType},
    Usecase,
};

use hmac::{Hmac, NewMac};
use jwt::{AlgorithmType, Header, SignWithKey, Token};
use sha2::Sha256;
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct Deps {
    account_repository: Arc<dyn AccountRepository>,
}

impl Deps {
    pub fn new(
        account_repository: Arc<dyn AccountRepository>,
    ) -> Self {
        Self { account_repository }
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
            // JWT Access TokenとRefresh Tokenを返す
            Some(_account) => {
                // - LoginとAccountをVerifyする。
                //
                let key: Hmac<Sha256> =
                    Hmac::new_varkey(b"horoscopes-secret").unwrap();
                let header: Header = Default::default();
                let mut claims = BTreeMap::new();
                claims.insert("user_id", "xxxxxxxx0001");

                let access_token: Token<
                    Header,
                    BTreeMap<&str, &str>,
                    _,
                > = Token::new(header, claims);
                let signed_access_token =
                    access_token.sign_with_key(&key).unwrap();

                // - Access TokenとRefresh Tokenを生成する。
                // - Refresh Tokenを保存する。
                return Ok(Output {
                    access_token: signed_access_token.into(),
                });
            }
        }
    }
}
