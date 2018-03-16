use futures::{self, Future};
use futures::future::FutureResult;

use hyper::{self, Get, Post, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Http, Service, Request, Response};

use request::BookRequest;
use errors::BookServiceError;
use dao::BookDao;

struct BookService;

impl Service for BookService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureResult<Response, hyper::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let dao = BookDao::new();
        futures::future::ok(
            match BookRequest::from_request(&req) {
                Ok(BookRequest::PostBook) => {
                    unimplemented!
                },
                Ok(BookRequest::GetBook(uuid)) => {
                    dao.get(uuid)
                        .and



                },
                Err(BookServiceError::NotFoundError) => {
                    Response::new()
                        .with_status(StatusCode::NotFound)
                },
                Err(BookServiceError::InvalidUuidError(pe)) => {
                    Response::new()
                        .with_status(StatusCode::BadRequest)
                }
            }
        )
    }




}

