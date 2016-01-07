#[macro_use]
extern crate rusoto;
extern crate uuid;

mod dao;
mod model;
mod dynamo_utils;

use dao::BookDao;
use rusoto::dynamodb::DynamoDBError;
use model::Book;
use uuid::Uuid;

fn main() {
    let mut dao = BookDao::new();

    let book0 = Book {
        book_id: Uuid::new_v4(),
        author: "Ernest Hemmingway".to_string(),
        title: "For Whom the Bell Tolls".to_string()
    };
    print_put(&mut dao, &book0);

    let book_result0: Result<Option<Book>, DynamoDBError> = dao.get(&book0.book_id);
    print_result(&book0.book_id, book_result0);
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
