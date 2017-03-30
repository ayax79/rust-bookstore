extern crate rusoto;
extern crate uuid;
extern crate iron;
#[macro_use(router)]
extern crate router;
extern crate hyper;
extern crate rustc_serialize;

#[macro_use]
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
use dynamo_utils::initialize_db;

fn main() {
    initialize_db();

    let router = router!(
        get_book: get "/books/:book_id" => get_book,
        get_books: get "/books" => book_list,
        put_book: put "/books/:book_id" => create_book
    );

    fn book_list(_: &mut Request) -> IronResult<Response> {
        let book0 = Book {
            book_id: Uuid::new_v4(),
            author: "Ernest Hemmingway".to_string(),
            title: "For Whom the Bell Tolls".to_string()
        };
        let payload = json::encode(&book0).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    fn get_book(request: &mut Request) -> IronResult<Response> {
        let mut dao = BookDao::new();
        let ref book_id_string = request.extensions.get::<Router>().unwrap().find("book_id").unwrap();
        let book_id = Uuid::parse_str(book_id_string).unwrap();
        let book:Book = dao.get(&book_id).unwrap().unwrap();
        let payload = json::encode(&book).unwrap();
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

    println!("Starting bookstore server on port 8080");
    Iron::new(router).http("0.0.0.0:8080").unwrap();
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