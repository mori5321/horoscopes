use crate::domain::entities::account::{Account, Login, SignUp};
use crate::domain::entities::user::ID as UserID;

pub trait AccountService: Send + Sync {
    fn is_duplicated(&self, signup: &SignUp) -> bool;
    fn from_signup(
        &self,
        signup: &SignUp,
        user_id: UserID,
    ) -> Account;
    fn verify(&self, login: &Login, account: &Account) -> bool;
}
