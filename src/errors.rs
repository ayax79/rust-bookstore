use std::fmt;
use std::error::Error;
use uuid::ParseError;
use rusoto_dynamodb::{PutItemError, GetItemError};


#[derive(Debug)]
pub enum BookServiceError {
    InvalidUuidError(ParseError),
    NotFoundError,
    BookCreateError(PutItemError),
    BookGetError(GetItemError)
}

impl fmt::Display for BookServiceError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BookServiceError::InvalidUuidError(pe) => {
                write!(f, "Root Cause: {}", pe)
            },
            BookServiceError::NotFoundError => {
                write!(f, "Resource or path was not found")
            },
            BookServiceError::BookCreateError(pie) => {
                write!(f, "Root Cause: {}", pie)
            },
            BookServiceError::BookGetError(gie) => {
                write!(f, "Root Cause: {}", gie)
            }
        }
    }
}

impl Error for BookServiceError {
    fn description(&self) -> &str {
        match *self {
            BookServiceError::InvalidUuidError(ref cause) => cause.description(),
            BookServiceError::NotFoundError => "Resource or path could was not found",
            BookServiceError::BookCreateError(ref cause) => cause.description(),
            BookServiceError::BookGetError(ref cause) => cause.description()
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            BookServiceError::InvalidUuidError(ref cause) => Some(cause),
            BookServiceError::BookCreateError(ref cause) => Some(cause),
            BookServiceError::BookGetError(ref cause) => Some(cause),
            _ => None

        }
    }

}