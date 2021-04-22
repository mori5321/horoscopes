use std::error::Error;
use std::fmt;
use warp::reject::Reject;
use crate::usecases::errors::{UsecaseError, UsecaseErrorType, BusinessError, SystemError};

#[derive(Debug)]
pub struct AppError {
    child: UsecaseError,
    pub err_type: AppErrorType,
    pub message: String,
}

#[derive(Debug)]
pub enum AppErrorType {
    BadRequest,
    NotFound,
    UnprocessableEntity,
    Internal,
}

pub fn from_usecase_error(err: UsecaseError) -> AppError {
    match err.err_type {
        UsecaseErrorType::BusinessError(ref business_error) => {
            match business_error {
                BusinessError::NotFoundError => {
                    AppError {
                        child: err.clone(),
                        err_type: AppErrorType::NotFound,
                        message: err.message.clone(),
                    }
                },
                BusinessError::ValidationError => {
                    AppError {
                        child: err.clone(),
                        err_type: AppErrorType::UnprocessableEntity,
                        message: err.message.clone(),
                    }
                },
            }
        },
        UsecaseErrorType::SystemError(ref system_error) => {
            match system_error {
                SystemError::UnknownError => {
                    AppError { 
                        child: err.clone(),
                        err_type: AppErrorType::Internal,
                        message: err.message.clone(),
                    }
                }
            }
        }
    }
}


impl Reject for AppError {}


impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.child)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

