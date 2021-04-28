use crate::domain::entities::account::{Account, Login, SignUp};

pub trait AccountService: Send + Sync {
    fn is_duplicated(&self, signup: &SignUp) -> bool;
    fn to_account(&self, signup: &SignUp) -> Account;
    fn verify(&self, login: &Login, account: &Account) -> bool;
}
