use std::error::Error;
use std::fmt;
use warp::reject::Reject;
use crate::usecases::errors::{UsecaseError, UsecaseErrorType};

#[derive(Debug)]
pub struct AppError {
    err: UsecaseError,
    err_type: AppErrorType,
}

#[derive(Debug)]
pub enum AppErrorType {
    ClientError,
    ApplicationError,
}

pub fn from_usecase_error(err: UsecaseError) -> AppError {
    match err.err_type {
        UsecaseErrorType::BusinessError => {
            AppError { err, err_type: AppErrorType::ClientError }
        },
        UsecaseErrorType::SystemError => { 
           AppError { err, err_type: AppErrorType::ApplicationError }
        }
    }
}


impl Reject for AppError {}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.err)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.err)
    }
}

