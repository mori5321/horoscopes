use crate::domain::entities::account;
use crate::domain::entities::todo;
use crate::domain::entities::user;

pub trait TodoRepository: Send + Sync {
    fn list(&self) -> Vec<todo::Todo>;
    fn find(&self, id: todo::ID) -> Option<todo::Todo>;
    fn store(&self, todo: todo::Todo) -> Result<(), String>; // ErrorはStringで仮置き
    fn remove(&self, id: todo::ID) -> Result<(), String>; // ErrorはStringで仮置き
}

pub trait AccountRepository: Send + Sync {
    fn find_by_email(
        &self,
        email: account::Email,
    ) -> Option<account::Account>;
    fn store(&self, account: account::Account) -> Result<(), String>; // ErrorはStringで仮置
}

pub trait UserRepository: Send + Sync {
    fn store(&self, user: user::User) -> Result<(), String>;
}
