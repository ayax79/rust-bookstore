use std::{fmt, io};
use std::error::Error;
use uuid::ParseError;
use rusoto_dynamodb::{PutItemError, GetItemError};
use serde_json::Error as SerdeJsonError;
use hyper::Error as HyperError;
use std::convert::From;

#[derive(Debug)]
pub enum BookServiceError {
    InvalidUuidError(ParseError),
    NotFoundError,
    /// Wrapper around rusoto put failure
    BookCreateError(PutItemError),
    /// Wrapper around rusoto get failure
    BookGetError(GetItemError),
    /// Wrapper for serde parsing errors
    BookParseError(SerdeJsonError),
    BookSerializationError(SerdeJsonError),
    /// generic hyper error wrapper
    BookBodyError(HyperError)
}

impl fmt::Display for BookServiceError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &BookServiceError::InvalidUuidError(ref pe) => {
                write!(f, "Root Cause: {}", pe)
            },
            &BookServiceError::NotFoundError => {
                write!(f, "Resource or path was not found")
            },
            &BookServiceError::BookCreateError(ref pie) => {
                write!(f, "Root Cause: {}", pie)
            },
            &BookServiceError::BookGetError(ref gie) => {
                write!(f, "Root Cause: {}", gie)
            },
            &BookServiceError::BookParseError(ref sje) => {
                write!(f, "Root Cause: {}", sje)
            },
            &BookServiceError::BookSerializationError(ref sje) => {
                write!(f, "Root Cause: {}", sje)
            },
            &BookServiceError::BookBodyError(ref he) => {
                write!(f, "Root Cause: {}", he)
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
            BookServiceError::BookGetError(ref cause) => cause.description(),
            BookServiceError::BookParseError(ref cause) => cause.description(),
            BookServiceError::BookSerializationError(ref cause) => cause.description(),
            BookServiceError::BookBodyError(ref cause) => cause.description()
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            BookServiceError::InvalidUuidError(ref cause) => Some(cause),
            BookServiceError::BookCreateError(ref cause) => Some(cause),
            BookServiceError::BookGetError(ref cause) => Some(cause),
            BookServiceError::BookParseError(ref cause) => Some(cause),
            BookServiceError::BookSerializationError(ref cause) => Some(cause),
            BookServiceError::BookBodyError(ref cause) => Some(cause),
            _ => None

        }
    }

}

impl From<HyperError> for BookServiceError {
    fn from(err: HyperError) -> Self {
        BookServiceError::BookBodyError(err)
    }
}

impl From<BookServiceError> for io::Error {
    fn from(err: BookServiceError) -> Self {
        io::Error::new(io::ErrorKind::Other, err)
    }
}
