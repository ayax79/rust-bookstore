use std::str;
use hyper::{self, StatusCode};
use hyper::header::ContentType;
use hyper::server::{Service, Request, Response};
use futures::{self, Stream, Future};
use log::Level;

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

                Box::new(futures::done(f))
            },
            Ok(BookRequest::PostBook) => {
                let f = req.body().concat2()
                    .map(|body| {
                        if log_enabled!(Level::Debug) {
                            debug!("body: {:?}", str::from_utf8(&body));
                        }

                        let mut dao = BookDao::new();
                        Book::from_slice(body.as_ref())
                            .and_then(|ref book| {
                                dao.put(book)
                                    .map(|_| StatusCode::Accepted)
                            })
                            .unwrap_or(StatusCode::BadRequest)
                    })
                    .map(|status_code| {
                        Response::new()
                            .with_status(status_code)
                    });
                Box::new(f)
            },
            Err(BookServiceError::NotFoundError) => {
                debug!("Path {} : NotFoundError", req.path());
                Box::new(futures::done(Ok(Response::new()
                    .with_status(StatusCode::NotFound))))
            },
            Err(_) => {
                debug!("Path {} : NotFoundError", req.path());
                Box::new(futures::done(Ok(bad_request())))
            }
        }
    }
}

fn bad_request() -> Response {
    Response::new().with_status(StatusCode::BadRequest)
}

