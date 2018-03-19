use futures::{self, Future};

use hyper::{self, StatusCode};
use hyper::header::ContentType;
use hyper::server::{Service, Request, Response};

use request::BookRequest;
use errors::BookServiceError;
use dao::BookDao;

struct BookService;

impl BookService {
    fn handle_request(&self, book_request: &BookRequest) -> Box<Future<Item=Response, Error=BookServiceError>> {
        let mut dao = BookDao::new();
        match book_request {
            &BookRequest::GetBook(uuid) => {
                let f = dao.get(&uuid)
                    .and_then(|book| book.to_vec())
                    .map(|v| {
                        Response::new()
                            .with_header(ContentType::json())
                            .with_body(v)
                    });
                Box::new(f)

            }
            &BookRequest::PostBook(book) => {
                let f = dao.put(&book)
                    .map(|_| {
                        Response::new()
                            .with_status(StatusCode::Accepted)
                    });
                Box::new(f)
            }
        }
    }

    fn handle_error(&self, err: &BookServiceError) -> Response {
        match err {
            &BookServiceError::NotFoundError => {
                Response::new()
                    .with_status(StatusCode::NotFound)
            }
            _ => {
                Response::new()
                    .with_status(StatusCode::BadRequest)
            }
        }
    }
}

impl Service for BookService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future =  Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let f = BookRequest::from_request(&req)
            .and_then(|ref req| self.handle_request(req))
            .or_else(|ref err| futures::done(Ok(self.handle_error(err))))
            .map_err(|err:BookServiceError| {
                // this should have already been handled
                // not sure if this the best return type, but it shouldn't happen anyways
                hyper::Error::Incomplete
            });
        Box::new(f)
    }
}

