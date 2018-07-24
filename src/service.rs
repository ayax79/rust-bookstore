use std::{str, io};
use std::error::Error;
use std::convert::From;
use hyper::{Body, Request, Response};
use hyper::header::CONTENT_TYPE;
use futures::{future, Stream, Future};
use log::Level;

use request::BookRequest;
use errors::BookServiceError;
use dao::BookDao;
use model::Book;
use settings::Settings;

type BookSvcFuture = Box<Future<Item=Response<Body>, Error=io::Error> + Send>;

pub fn book_service(req: Request<Body>) -> BookSvcFuture {
    debug!("Received: {} {}", req.method(), &req.uri().path());

    if let Ok(settings) = Settings::new() {
        match BookRequest::from_request(&req) {
            Ok(BookRequest::GetBook(uuid)) => {
                let result = BookDao::new(&settings)
                    .and_then(|dao| dao.get(&uuid))
                    .and_then(|book| book.to_vec())
                    .map(Body::from)
                    .map(|v| {
                        Response::builder()
                            .header(CONTENT_TYPE, "application/json")
                            .body(v)
                            .unwrap()
                    })
                    .or_else(|err: BookServiceError| server_error(err.description()))
                    .map_err(From::from);

                Box::new(future::result(result))
            }
            Ok(BookRequest::PostBook) => {
                let s = settings.clone();
                let f = req.into_body().concat2()
                    .map(move |body| {
                        if log_enabled!(Level::Debug) {
                            debug!("body: {:?}", str::from_utf8(body.as_ref()));
                        }

                        Book::from_slice(body.as_ref())
                            .and_then(|ref book| {
                                BookDao::new(&s)
                                    .and_then(|dao| dao.put(book))
                                    .map(|_| 202)
                            })
                            .unwrap_or(400)
                    })
                    .map(|status_code| {
                        Response::builder()
                            .status(status_code)
                            .body(Body::empty())
                            .unwrap()
                    })
                    .or_else(|err| server_error(err.description()))
                    .map_err(From::from);
                Box::new(f)
            }
            Ok(BookRequest::Health) => {
                let response = Response::builder()
                    .status(200)
                    .body(Body::empty())
                    .unwrap();
                Box::new(future::ok(response))
            }
            Err(BookServiceError::NotFoundError) => {
                debug!("Path {} : NotFoundError", req.uri().path());
                Box::new(future::ok(
                    Response::builder()
                        .status(404)
                        .body(Body::empty())
                        .unwrap()))
            }
            Err(_) => {
                debug!("Path {} : NotFoundError", req.uri().path());
                Box::new(future::ok(bad_request()))
            }
        }
    } else {
        let result = server_error("Could not load settings").map_err(From::from);
        Box::new(future::result(result))
    }
}

fn bad_request() -> Response<Body> {
    Response::builder()
        .status(400)
        .body(Body::empty())
        .unwrap()
}

fn server_error(msg: &str) -> Result<Response<Body>, BookServiceError> {
    Ok(Response::builder()
        .status(500)
        .body(Body::from(msg.to_string()))
        .unwrap())
}