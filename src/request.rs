use errors::BookServiceError;
use hyper::{Method, Request};
use uuid::Uuid;
use model::Book;
use futures::{self, Future, Stream};


#[derive(Debug, PartialEq)]
pub enum BookRequest {
    GetBook(Uuid),
    PostBook(Book),
}

//todo - move to TryFrom when available
impl BookRequest {
    pub fn from_request(req: &Request) -> Box<Future<Item=BookRequest, Error=BookServiceError> >{
        match (req.method(), req.path()) {
            (&Method::Get, _) => Self::handle_get(req),
            (&Method::Post, "/book") => Self::handle_post(req),
            _ => Box::new(futures::done(Err(BookServiceError::NotFoundError)))
        }
    }

    fn handle_get(req: &Request) -> Box<Future<Item=BookRequest, Error=BookServiceError>> {
        let path = req.path();
        let result = if path.starts_with("/book/") {
            Self::parse_id(req)
                .map(|uuid| BookRequest::GetBook(uuid))
        } else {
            Err(BookServiceError::NotFoundError)
        };
        Box::new(futures::done(result))
    }

    fn handle_post(req: &Request) -> Box<Future<Item=BookRequest, Error=BookServiceError>> {
        let future = req.body().concat2()
            .map_err(BookServiceError::from)
            .and_then(|body| {
                Book::from_slice(body.as_ref())
            })
            .map(BookRequest::PostBook);
        Box::new(future)
    }

    /// Parses the uuid off the request path
    fn parse_id(req: &Request) -> Result<Uuid, BookServiceError> {
        let path = req.path().to_owned();
        path.rfind('/')
            .ok_or(BookServiceError::NotFoundError)
            .map(|index| path[(index + 1)..].to_owned())
            .and_then(|ref sub_string| Uuid::parse_str(sub_string).map_err(BookServiceError::InvalidUuidError))
    }
}

#[cfg(test)]
mod tests {
    use hyper::{Request, Method};
    use super::*;

    #[test]
    fn test_post() {
        let request = Request::new(Method::Post, FromStr::from_str("/book").unwrap());
        let request_type = BookRequest::from_request(&request).unwrap();
        assert_eq!(BookRequest::PostBook, request_type);
    }

    #[test]
    fn test_get() {
        let uuid = Uuid::new_v4();
        let url_str = "/book/".to_owned() + uuid.hyphenated().to_string().as_ref();

        let request = Request::new(Method::Get, FromStr::from_str(&url_str).unwrap());
        let request_type = BookRequest::from_request(&request).unwrap();
        assert_eq!(BookRequest::GetBook(uuid), request_type);
    }
}
