use uuid::{Uuid, ParseError};

use rocket::request::FromParam;
use rocket::http::RawStr;


/// A wrapper to get around E0117 - only traits defined in the current crate can be implemented for arbitrary types
pub struct UuidWrapper(pub Uuid);

impl <'a> FromParam<'a> for UuidWrapper {
    type Error = ParseError;

    fn from_param(param: &'a RawStr) -> Result<Self, Self::Error> {
        Uuid::parse_str(param).map(|uuid| UuidWrapper(uuid))
    }
}
