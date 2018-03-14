use std::fmt;
use std::error::Error;
use uuid::ParseError;


#[derive(Debug)]
pub enum BookServiceError {
    InvalidUuidError(ParseError),
    NotFoundError
}

impl fmt::Display for BookServiceError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BookServiceError::InvalidUuidError(pe) => {
                write!(f, "Root Cause: {}", pe)
            },
            BookServiceError::NotFoundError => {
                write!(f, "Resource or path was not found")
            }
        }
    }
}

impl Error for BookServiceError {
    fn description(&self) -> &str {
        match *self {
            BookServiceError::InvalidUuidError(ref cause) => cause.description(),
            BookServiceError::NotFoundError => "Resource or path could was not found"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            BookServiceError::InvalidUuidError(ref cause) => Some(cause),
            _ => None

        }
    }

}