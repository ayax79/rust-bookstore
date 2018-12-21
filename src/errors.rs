use config::ConfigError;
use hyper::Error as HyperError;
use r2d2_redis::r2d2::Error as R2D2RedisError;
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
    BookCreateError(DaoCause),
    /// Wrapper around redis get failure
    BookGetError(DaoCause),
    /// Wrapper for serde parsing errors
    BookParseError(SerdeJsonError),
    BookSerializationError(SerdeJsonError),
    /// generic hyper error wrapper
    BookBodyError(HyperError),
    DaoInitializationError(DaoCause),
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
            &BookServiceError::BookCreateError(ref pie) => {
                write!(f, "Root Cause: {:?}", pie.cause())
            }
            &BookServiceError::BookGetError(ref gie) => write!(f, "Root Cause: {:?}", gie.cause()),
            &BookServiceError::BookParseError(ref sje) => write!(f, "Root Cause: {}", sje),
            &BookServiceError::BookSerializationError(ref sje) => write!(f, "Root Cause: {}", sje),
            &BookServiceError::BookBodyError(ref he) => write!(f, "Root Cause: {}", he),
            &BookServiceError::DaoInitializationError(ref e) => {
                write!(f, "Root Cause: {:?}", e.cause())
            }
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

    fn source(&self) -> Option<&(Error + 'static)> {
        match self {
            BookServiceError::InvalidUuidError(cause) => Some(cause),
            BookServiceError::BookCreateError(cause) => cause.cause(),
            BookServiceError::BookGetError(cause) => cause.cause(),
            BookServiceError::BookParseError(cause) => Some(cause),
            BookServiceError::BookSerializationError(cause) => Some(cause),
            BookServiceError::BookBodyError(cause) => Some(cause),
            BookServiceError::DaoInitializationError(cause) => cause.cause(),
            BookServiceError::SettingsError(cause) => Some(cause),
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

impl From<R2D2RedisError> for BookServiceError {
    fn from(err: R2D2RedisError) -> Self {
        BookServiceError::DaoInitializationError(DaoCause(None, Some(err)))
    }
}

impl From<RedisError> for BookServiceError {
    fn from(err: RedisError) -> Self {
        BookServiceError::DaoInitializationError(DaoCause(Some(err), None))
    }
}

#[derive(Debug)]
pub struct DaoCause(Option<RedisError>, Option<R2D2RedisError>);

impl DaoCause {
    pub fn cause(&self) -> Option<&(Error + 'static)> {
        match self {
            DaoCause(Some(ref e), _) => Some(e),
            DaoCause(_, Some(e)) => Some(e),
            _ => None,
        }
    }

    pub fn description(&self) -> &str {
        self.cause().map(|d| d.description()).unwrap_or("")
    }
}

impl From<RedisError> for DaoCause {
    fn from(e: RedisError) -> Self {
        DaoCause(Some(e), None)
    }
}

impl From<R2D2RedisError> for DaoCause {
    fn from(e: R2D2RedisError) -> Self {
        DaoCause(None, Some(e))
    }
}
