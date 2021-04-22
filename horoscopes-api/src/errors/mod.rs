use std::fmt;
use warp::reject::Reject;

#[derive(Debug)]
pub enum ErrorType {
    BadRequest,
    NotFound,
    Internal,
}

#[derive(Debug)]
pub struct AppError {
    pub err_type: ErrorType,
    pub message: String,
}

impl std::error::Error for AppError {}
impl Reject for AppError {}
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

// Errorsは各レイヤーごとにあるのがよい...?
