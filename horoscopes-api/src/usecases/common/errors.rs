use crate::domain::errors::{DomainError, DomainErrorType};
use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct UsecaseError {
    child: Option<Arc<dyn Error + Sync + Send>>,
    message: String,
    err_type: UsecaseErrorType,
}

impl UsecaseError {
    pub fn new(err_type: UsecaseErrorType, message: String) -> Self {
        Self {
            child: None,
            message,
            err_type,
        }
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }

    pub fn err_type(&self) -> UsecaseErrorType {
        self.err_type.clone()
    }
}

pub fn from_domain_error(err: DomainError) -> UsecaseError {
    match err.err_type() {
        DomainErrorType::ExceedMaxLengthError => UsecaseError {
            child: Some(Arc::new(err.clone())),
            message: err.message(),
            err_type: UsecaseErrorType::BusinessError(
                BusinessError::ValidationError,
            ),
        },
    }
}

#[derive(Debug, Clone)]
pub enum UsecaseErrorType {
    BusinessError(BusinessError),
    SystemError(SystemError),
}

#[derive(Debug, Clone)]
pub enum BusinessError {
    ValidationError,
    DuplicatedError,
    NotFoundError,
}

#[derive(Debug, Clone)]
pub enum SystemError {
    UnknownError,
}

impl Error for UsecaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self.child.as_ref() {
            Some(err) => {
                let e = Arc::as_ref(&err);
                return Some(e.clone());
            }
            None => None,
        }
    }
}
impl fmt::Display for UsecaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
