use crate::domain::entities::{
    account, diagnosis, organization, todo, user,
};

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

pub trait OrganizationRepository: Send + Sync {
    fn store(
        &self,
        organization: &organization::Organization,
    ) -> Result<(), String>;
}

pub trait DiagnosisRepository: Send + Sync {
    fn list(&self) -> Option<Vec<diagnosis::Diagnosis>>;
    // fn find(&self) -> Option<diagnosis::Diagnosis>;
    fn store(
        &self,
        diagnosis: &diagnosis::Diagnosis,
    ) -> Result<(), String>;
}
