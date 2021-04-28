use crate::domain::entities::account::{Account, Login, SignUp};
use crate::domain::repositories::AccountRepository;
use crate::domain::services::account_service::AccountService;

use argon2::{self, Config};
use std::sync::Arc;

pub struct AccountServiceImpl {
    repo: Arc<dyn AccountRepository>,
}

impl AccountServiceImpl {
    pub fn new(repo: Arc<dyn AccountRepository>) -> Self {
        Self { repo }
    }
}

const SALT: &[u8; 10] = b"horoscopes";

impl AccountService for AccountServiceImpl {
    fn to_account(&self, signup: &SignUp) -> Account {
        let config = Config::default();
        let salt = SALT;
        let password = signup.password().value();

        let hash =
            argon2::hash_encoded(&password.as_bytes(), salt, &config)
                .expect("Failed to hash password.");

        Account::from_signup(&signup, hash)
    }

    fn verify(&self, login: &Login, account: &Account) -> bool {
        let account_password_hash = account.password_hash().value();
        let login_password = login.password().value();

        let matched = argon2::verify_encoded(
            &account_password_hash,
            &login_password.as_bytes(),
        );

        match matched {
            Ok(boolean) => boolean,
            Err(_) => false,
        }
    }

    fn is_duplicated(&self, signup: &SignUp) -> bool {
        let opt_account = self.repo.find_by_email(signup.email());
        match opt_account {
            Some(_) => true,
            None => false,
        }
    }
}
