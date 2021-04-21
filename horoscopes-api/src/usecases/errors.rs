use std::error::Error;
use std::fmt;
use crate::domain::errors::{DomainError, DomainErrorType};
use std::sync::Arc;
use std::fmt::Debug;

// trait UsecaseChildError: Send + Sync {}
// 
// impl UsecaseChildError for DomainError {}
// 
// type UsecaseChildError = DomainError


#[derive(Debug)]
pub struct UsecaseError {
    pub child: Option<DomainError>,
    pub message: String,
    pub err_type: UsecaseErrorType, 
}

pub fn from_domain_error(err: DomainError) -> UsecaseError {
    match err.err_type {
        DomainErrorType::ExceedMaxLengthError => {
            UsecaseError {
                child: Some(err.clone()),
                message: err.message,
                err_type: UsecaseErrorType::BusinessError
            }
        }
        _ => {
            UsecaseError {
                child: Some(err.clone()),
                message: err.message,
                err_type: UsecaseErrorType::SystemError,
            }
        }
    }
}

#[derive(Debug)]
pub enum UsecaseErrorType {
    BusinessError,
    SystemError,
}

impl Error for UsecaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self.child.as_ref() {
            Some(err) => Some(err),
            None => None
        }
    }
}
impl fmt::Display for UsecaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
