// use fmt::Display;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct DomainError {
    pub err_type: DomainErrorType,
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum DomainErrorType {
    ExceedMaxLengthError,
}

impl Error for DomainError {}
impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {}", self.err_type, self.message)
    }
}

// #[derive(Debug)]
// pub struct ExceedMaxLengthError {
//     pub text: String,
//     pub length: usize,
//     pub max_length: usize,
// }

// impl DomainError for ExceedMaxLengthError {}

// impl fmt::Display for ExceedMaxLengthError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "Domain Error: Exceeded max length. LENGTH: {}. MAX_LENGTH: {}.",
//             self.length,
//             self.max_length
//         )
//     }
// }

// impl Error for ExceedMaxLengthError {}
