use futures::Future;

use hyper::{self, StatusCode};
use hyper::header::ContentType;
use hyper::server::{Service, Request, Response};
use futures::{self, Stream};

use request::BookRequest;
use errors::BookServiceError;
use dao::BookDao;
use model::Book;

pub struct BookService;

impl Service for BookService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        debug!("Received: {} {}", req.method(), req.path());
        match BookRequest::from_request(&req) {
            Ok(BookRequest::GetBook(uuid)) => {
                let mut dao = BookDao::new();
                let f = dao.get(&uuid)
                    .and_then(|book| book.to_vec())
                    .map(|v| {
                        Response::new()
                            .with_header(ContentType::json())
                            .with_body(v)
                    })
                    .map_err(From::from);

                Box::new(f)
            },
            Ok(BookRequest::PostBook) => {
                let f = req.body().concat2()
                    .and_then(|body| {
                        debug!("body: {:?}", &body);
                        let mut dao = BookDao::new();
                        Book::from_slice(body.as_ref())
                            .map(|ref book| dao.put(book))
                            .map_err(From::from)
                    })
                    .map(|_| {
                        Response::new()
                            .with_status(StatusCode::Accepted)
                    });
                Box::new(f)
            },
            Err(BookServiceError::NotFoundError) => {
                Box::new(futures::done(Ok(Response::new()
                    .with_status(StatusCode::NotFound))))
            },
            Err(_) => {
                Box::new(futures::done(Ok(Response::new()
                    .with_status(StatusCode::BadRequest))))
            }
        }
    }
}

