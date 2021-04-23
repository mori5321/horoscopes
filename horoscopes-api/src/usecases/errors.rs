use std::error::Error;
use std::fmt;
use crate::domain::errors::{DomainError, DomainErrorType};
use std::sync::Arc;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct UsecaseError {
    pub child: Option<Arc<dyn Error + Sync + Send>>,
    pub message: String,
    pub err_type: UsecaseErrorType, 
}

pub fn from_domain_error(err: DomainError) -> UsecaseError {
    match err.err_type {
        DomainErrorType::ExceedMaxLengthError => {
            UsecaseError {
                child: Some(Arc::new(err.clone())),
                message: err.message.to_string(),
                err_type: UsecaseErrorType::BusinessError(BusinessError::ValidationError)
            }
        }
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
    NotFoundError
}

#[derive(Debug, Clone)]
pub enum SystemError {
   UnknownError 
}

impl Error for UsecaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self.child.as_ref() {
            Some(err) => {
                let e = Arc::as_ref(&err);
                return Some(e.clone());
            },
            None => None
        }
    }
}
impl fmt::Display for UsecaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
