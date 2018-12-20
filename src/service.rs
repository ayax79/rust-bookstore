use futures::{future, Future, Stream};
use hyper::header::CONTENT_TYPE;
use hyper::{Body, Request, Response};
use std::convert::From;
use std::error::Error;
use std::{io, str};

use dao::BookDao;
use errors::BookServiceError;
use model::Book;
use request::BookRequest;
use settings::Settings;

type BookSvcFuture = Box<Future<Item = Response<Body>, Error = io::Error> + Send>;

pub struct BookService {
    settings: Settings,
}

impl BookService {
    pub fn new(settings: &Settings) -> Self {
        BookService {
            settings: settings.to_owned(),
        }
    }

    pub fn service(&self, req: Request<Body>) -> BookSvcFuture {
        println!("Received: {} {}", req.method(), &req.uri().path());

        match BookRequest::from_request(&req) {
            Ok(BookRequest::GetBook(uuid)) => {
                println!("Retrieving GET {}", &uuid);
                let result = BookDao::new(&self.settings)
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
                println!("Processing POST - creating book");
                let s = self.settings.to_owned();
                let f = req
                    .into_body()
                    .concat2()
                    .map(move |body| {
                        println!("POST body {:?}", str::from_utf8(body.as_ref()));

                        Book::from_slice(body.as_ref())
                            .and_then(|ref book| {
                                BookDao::new(&s).and_then(|dao| dao.put(book)).map(|_| 202)
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
                println!("Processing health request");
                let response = Response::builder().status(200).body(Body::empty()).unwrap();
                Box::new(future::ok(response))
            }
            Err(BookServiceError::NotFoundError) => {
                debug!("Path {} : NotFoundError", req.uri().path());
                Box::new(future::ok(
                    Response::builder().status(404).body(Body::empty()).unwrap(),
                ))
            }
            Err(_) => {
                debug!("Path {} : NotFoundError", req.uri().path());
                Box::new(future::ok(bad_request()))
            }
        }
    }
}

fn bad_request() -> Response<Body> {
    Response::builder().status(400).body(Body::empty()).unwrap()
}

fn server_error(msg: &str) -> Result<Response<Body>, BookServiceError> {
    Ok(Response::builder()
        .status(500)
        .body(Body::from(msg.to_string()))
        .unwrap())
}
