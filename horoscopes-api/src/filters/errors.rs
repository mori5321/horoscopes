use crate::usecases::common::errors::{
    BusinessError, SystemError, UsecaseError, UsecaseErrorType,
};
use std::error::Error;
use std::fmt;
use warp::reject::Reject;

#[derive(Debug)]
pub struct AppError {
    // BoxとArcどっちがいいの?
    child: Option<Box<dyn Error + Sync + Send>>,
    pub err_type: AppErrorType,
    pub message: String,
}

#[derive(Debug, PartialEq)]
pub enum AppErrorType {
    BadRequest,
    NotFound,
    UnprocessableEntity,
    Internal,
}

pub fn from_usecase_error(err: UsecaseError) -> AppError {
    match err.err_type() {
        UsecaseErrorType::BusinessError(ref business_error) => {
            match business_error {
                BusinessError::NotFoundError => AppError {
                    child: Some(Box::new(err.clone())),
                    err_type: AppErrorType::NotFound,
                    message: err.message(),
                },
                BusinessError::ValidationError => AppError {
                    child: Some(Box::new(err.clone())),
                    err_type: AppErrorType::UnprocessableEntity,
                    message: err.message(),
                },
                BusinessError::DuplicatedError => AppError {
                    child: Some(Box::new(err.clone())),
                    err_type: AppErrorType::BadRequest,
                    message: err.message(),
                },
            }
        }
        UsecaseErrorType::SystemError(ref system_error) => {
            match system_error {
                SystemError::UnknownError => AppError {
                    child: Some(Box::new(err.clone())),
                    err_type: AppErrorType::Internal,
                    message: err.message(),
                },
            }
        }
    }
}

impl Reject for AppError {}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // Some(&self.child)
        match &self.child {
            Some(err) => {
                let e = err.as_ref();
                return Some(e.clone());
            }
            None => Some(self),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
