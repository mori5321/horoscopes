use crate::domain::entities::account::{Account, Login, SignUp};

pub trait AccountService: Send + Sync {
    fn is_duplicated(&self, signup: &SignUp) -> bool;
    fn from_signup(&self, signup: &SignUp) -> Account;
    fn verify(&self, login: &Login, account: &Account) -> bool;
}
