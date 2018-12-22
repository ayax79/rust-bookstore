use crate::errors::BookServiceError;
use hyper::{Body, Method, Request};
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub enum BookRequest {
    GetBook(Uuid),
    PostBook,
    Health,
}

//todo - move to TryFrom when available
impl BookRequest {
    pub fn from_request(req: &Request<Body>) -> Result<BookRequest, BookServiceError> {
        match (req.method(), req.uri().path()) {
            (&Method::GET, _) => Self::handle_get(req),
            (&Method::POST, "/book/") => Self::handle_post(),
            _ => Err(BookServiceError::NotFoundError),
        }
    }

    fn handle_get(req: &Request<Body>) -> Result<BookRequest, BookServiceError> {
        let path = req.uri().path();
        if path.starts_with("/book/health") {
            Ok(BookRequest::Health)
        } else if path.starts_with("/book/") {
            Self::parse_id(req).map(|uuid| BookRequest::GetBook(uuid))
        } else {
            Err(BookServiceError::NotFoundError)
        }
    }

    fn handle_post() -> Result<BookRequest, BookServiceError> {
        Ok(BookRequest::PostBook)
    }

    /// Parses the uuid off the request path
    fn parse_id(req: &Request<Body>) -> Result<Uuid, BookServiceError> {
        let path = req.uri().path().to_owned();
        path.rfind('/')
            .ok_or(BookServiceError::NotFoundError)
            .map(|index| path[(index + 1)..].to_owned())
            .and_then(|ref sub_string| {
                Uuid::parse_str(sub_string).map_err(BookServiceError::InvalidUuidError)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hyper::Request;

    #[test]
    fn test_post() {
        let request = Request::builder()
            .method("POST")
            .uri("/book/")
            .body(Body::empty())
            .unwrap();
        let request_type = BookRequest::from_request(&request).unwrap();
        assert_eq!(BookRequest::PostBook, request_type);
    }

    #[test]
    fn test_get() {
        let uuid = Uuid::new_v4();
        let url_str = "/book/".to_owned() + uuid.hyphenated().to_string().as_ref();

        let request = Request::builder()
            .method("GET")
            .uri(url_str)
            .body(Body::empty())
            .unwrap();

        let request_type = BookRequest::from_request(&request).unwrap();
        assert_eq!(BookRequest::GetBook(uuid), request_type);
    }
}
