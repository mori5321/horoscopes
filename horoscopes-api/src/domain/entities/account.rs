use super::user::User;
use crate::domain::errors::{DomainError, DomainErrorType};

#[derive(Clone, Debug)]
pub struct Account {
    id: ID,
    email: Email,
    password_hash: PasswordHash,
    user: User,
}

impl Account {
    pub fn new(
        id: String,
        email: String,
        password_hash: String,
        user_id: String,
    ) -> Result<Self, DomainError> {
        let email = Email::new(email)?;

        Ok(Self {
            id: ID::new(id),
            email,
            password_hash: PasswordHash::new(password_hash),
            user: User::new(user_id),
        })
    }

    pub fn from_signup(
        signup: &SignUp,
        password_hash: String,
        user: User,
    ) -> Self {
        Account {
            id: signup.id.clone(),
            email: signup.email.clone(),
            password_hash: PasswordHash(password_hash),
            user,
        }
    }

    pub fn id(&self) -> ID {
        self.id.clone()
    }

    pub fn email(&self) -> Email {
        self.email.clone()
    }

    pub fn password_hash(&self) -> PasswordHash {
        self.password_hash.clone()
    }

    pub fn user(&self) -> &User {
        &self.user
    }
}

impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Clone, Debug)]
pub struct SignUp {
    id: ID,
    email: Email,
    password: Password,
    password_confirmation: Password,
}

#[derive(Clone, Debug)]
pub struct Login {
    email: Email,
    password: Password,
}

impl SignUp {
    pub fn new(
        id: String,
        email: String,
        password: String,
        password_confirmation: String,
    ) -> Result<Self, DomainError> {
        let email = Email::new(email)?;

        let account = Self {
            id: ID::new(id),
            email,
            password: Password::new(password),
            password_confirmation: Password::new(
                password_confirmation,
            ),
        };

        Ok(account)
    }

    pub fn email(&self) -> Email {
        self.email.clone()
    }

    pub fn password(&self) -> Password {
        self.password.clone()
    }

    pub fn password_confirmation(&self) -> Password {
        self.password_confirmation.clone()
    }
}

impl Login {
    pub fn new(
        email: String,
        password: String,
    ) -> Result<Self, DomainError> {
        let email = Email::new(email)?;

        Ok(Self {
            email,
            password: Password::new(password),
        })
    }

    pub fn email(&self) -> Email {
        self.email.clone()
    }

    pub fn password(&self) -> Password {
        self.password.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ID(String);

impl ID {
    fn new(id: String) -> Self {
        ID(id)
    }

    pub fn value(&self) -> String {
        let Self(s) = self;
        s.clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Email(String);

const EMAIL_MAX_LENGTH: usize = 256;

impl Email {
    fn new(email: String) -> Result<Self, DomainError> {
        if email.len() > EMAIL_MAX_LENGTH {
            let error = DomainError::new(
                DomainErrorType::ExceedMaxLengthError,
                format!(
                    "Email length should be less than {}",
                    EMAIL_MAX_LENGTH
                ),
            );
            return Err(error);
        }

        Ok(Email(email))
    }

    pub fn value(&self) -> String {
        let Self(s) = self;
        return s.clone();
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Password(String);

impl Password {
    fn new(password: String) -> Self {
        Password(password)
    }

    pub fn value(&self) -> String {
        let Password(password) = self;
        password.clone()
    }
}

#[derive(Clone, Debug)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(password_hash: String) -> Self {
        PasswordHash(password_hash)
    }

    pub fn value(&self) -> String {
        let PasswordHash(hash) = self;
        hash.clone()
    }
}
