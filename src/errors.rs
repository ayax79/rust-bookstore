use config::ConfigError;
use hyper::Error as HyperError;
use redis::RedisError;
use serde_json::Error as SerdeJsonError;
use std::convert::From;
use std::error::Error;
use std::{fmt, io};
use uuid::ParseError;

#[derive(Debug)]
pub enum BookServiceError {
    InvalidUuidError(ParseError),
    NotFoundError,
    /// Wrapper around redis put failure
    BookCreateError(RedisError),
    /// Wrapper around redis get failure
    BookGetError(RedisError),
    /// Wrapper for serde parsing errors
    BookParseError(SerdeJsonError),
    BookSerializationError(SerdeJsonError),
    /// generic hyper error wrapper
    BookBodyError(HyperError),
    DaoInitializationError(RedisError),
    MissingFieldError(String),
    SettingsError(ConfigError),
    RedisHostError,
    RedisPortError,
}

impl fmt::Display for BookServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &BookServiceError::InvalidUuidError(ref pe) => write!(f, "Root Cause: {}", pe),
            &BookServiceError::NotFoundError => write!(f, "Resource or path was not found"),
            &BookServiceError::BookCreateError(ref pie) => write!(f, "Root Cause: {}", pie),
            &BookServiceError::BookGetError(ref gie) => write!(f, "Root Cause: {}", gie),
            &BookServiceError::BookParseError(ref sje) => write!(f, "Root Cause: {}", sje),
            &BookServiceError::BookSerializationError(ref sje) => write!(f, "Root Cause: {}", sje),
            &BookServiceError::BookBodyError(ref he) => write!(f, "Root Cause: {}", he),
            &BookServiceError::DaoInitializationError(ref e) => write!(f, "Root Cause: {}", e),
            &BookServiceError::MissingFieldError(ref field) => {
                write!(f, "Invalid Book, missing field {} ", field)
            }
            &BookServiceError::SettingsError(ref e) => {
                write!(f, "Configuration Issue - Root Cause: {}", e)
            }
            &BookServiceError::RedisHostError => write!(f, "Redis host was missing"),
            &BookServiceError::RedisPortError => write!(f, "Redis port was missing"),
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
            BookServiceError::BookBodyError(ref cause) => cause.description(),
            BookServiceError::DaoInitializationError(ref cause) => cause.description(),
            BookServiceError::SettingsError(ref cause) => cause.description(),
            BookServiceError::RedisHostError => "Redis host was missing",
            BookServiceError::RedisPortError => "Redis port was missing",
            BookServiceError::MissingFieldError(_) => "Book entry was missing a field",
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
            BookServiceError::DaoInitializationError(ref cause) => Some(cause),
            BookServiceError::SettingsError(ref cause) => Some(cause),
            _ => None,
        }
    }
}

impl From<HyperError> for BookServiceError {
    fn from(err: HyperError) -> Self {
        BookServiceError::BookBodyError(err)
    }
}

impl From<ParseError> for BookServiceError {
    fn from(err: ParseError) -> Self {
        BookServiceError::InvalidUuidError(err)
    }
}

impl From<ConfigError> for BookServiceError {
    fn from(err: ConfigError) -> Self {
        BookServiceError::SettingsError(err)
    }
}

impl From<BookServiceError> for io::Error {
    fn from(err: BookServiceError) -> Self {
        io::Error::new(io::ErrorKind::Other, err)
    }
}
