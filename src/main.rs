#[macro_use]
extern crate rusoto;
extern crate uuid;
extern crate iron;
extern crate router;
extern crate rustc_serialize;

mod dynamo_utils;
mod model;
mod dao;

use std::io::Read;
use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json;
use model::Book;
use uuid::Uuid;
use dao::BookDao;
use rusoto::dynamodb::DynamoDBError;

// mod dao;
// mod model;
//
// use dao::BookDao;
// use rusoto::dynamodb::DynamoDBError;

fn main() {
    let mut router = Router::new();
    router.get("/books", book_list);
    router.put("/books", create_book);


    fn book_list(_: &mut Request) -> IronResult<Response> {
        let book0 = Book {
            book_id: Uuid::new_v4(),
            author: "Ernest Hemmingway".to_string(),
            title: "For Whom the Bell Tolls".to_string()
        };
        let payload = json::encode(&book0).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    fn create_book(request: &mut Request) -> IronResult<Response> {
        let mut dao = BookDao::new();
        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();
        let book: Book = json::decode(&payload).unwrap();
        print_put(&mut dao, &book);
        Ok(Response::with((status::Ok, payload)))
    }

    println!("Starting bookstore server on port 3000");
    Iron::new(router).http("localhost:3000").unwrap();
}

fn print_put(dao: &mut BookDao, book: &Book) -> () {
    match dao.put(book) {
        Ok(_) => {
            println!("Book {:#?} was added", book.title);
        }
        Err(err) => {
            println!("Could not insert book {:#?} because of {:#?}", book.title, err);
        }
    }
}

fn print_result(book_id: &Uuid, result: Result<Option<Book>, DynamoDBError>) -> () {
    match result {
        Ok(maybe_book) => {
            match maybe_book {
                Some(book) => {
                    println!("Found book {:#?}", book);
                }
                None => {
                    println!("No book was found under id {:?}", book_id);
                }
            }
        }
        Err(err) => {
            println!("An error occurred {:#?}", err);
        }
    }
}


// fn main() {
//     let mut dao = BookDao::new();
//
//     let book0 = Book {
//         book_id: Uuid::new_v4(),
//         author: "Ernest Hemmingway".to_string(),
//         title: "For Whom the Bell Tolls".to_string()
//     };
//     print_put(&mut dao, &book0);
//
//     let book_result0: Result<Option<Book>, DynamoDBError> = dao.get(&book0.book_id);
//     print_result(&book0.book_id, book_result0);
// }
