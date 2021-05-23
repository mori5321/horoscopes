use crate::adapters::infrastructure::services::account_service::AccountServiceImpl;
use crate::domain::entities::account::{self, Account, SignUp};
use crate::domain::entities::user::User;
use crate::domain::repositories::AccountRepository;
use crate::domain::services::account_service::AccountService;

use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

pub struct AccountRepositoryOnMemory {}

pub static ACCOUNTS_ON_MEMORY: Lazy<Mutex<Vec<Account>>> =
    Lazy::new(|| {
        let signup = SignUp::new(
            "account-0001".to_string(),
            "moss@hoge.com".to_string(),
            "password".to_string(),
            "password".to_string(),
        )
        .unwrap();

        let service = AccountServiceImpl::new(Arc::new(
            AccountRepositoryOnMemory::new(),
        ));
        let account = service
            .from_signup(&signup, User::new("user-0001".to_string()));

        println!("Account: {:?}", account);
        let accounts = vec![account];
        Mutex::new(accounts)
    });

impl AccountRepositoryOnMemory {
    pub fn new() -> Self {
        Self {}
    }
}

impl AccountRepository for AccountRepositoryOnMemory {
    fn find_by_email(
        &self,
        email: account::Email,
    ) -> Option<Account> {
        let accounts = ACCOUNTS_ON_MEMORY.lock().unwrap().clone();
        // TODO: accountの比較ができるようにしてあげたい。
        accounts
            .into_iter()
            .find(|account| account.email() == email)
    }

    fn store(&self, account: Account) -> Result<(), String> {
        let mut accounts = ACCOUNTS_ON_MEMORY.lock().unwrap();

        let opt_idx =
            accounts.clone().into_iter().position(|a| a == account);

        match opt_idx {
            Some(idx) => {
                accounts.splice(
                    idx..idx + 1,
                    vec![account].iter().cloned(),
                );
                Ok(())
            }
            None => {
                accounts.push(account.clone());
                Ok(())
            }
        }
    }
}
