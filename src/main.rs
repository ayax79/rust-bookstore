#[macro_use]
extern crate rusoto;
extern crate uuid;

mod dao;
mod model;
mod dynamo_utils;

use dao::MyDao;
use dynamo_utils::{create_db_helper, BOOKS_TABLE};
use rusoto::dynamodb::{DynamoDBError, DynamoDBHelper};
use model::BookEntry;
use uuid::Uuid;

fn main() {
    let mut dao = MyDao::new();

    let book0 = BookEntry {
        book_id: Uuid::new_v4(),
        author: "Ernest Hemmingway".to_string(),
        title: "For Whom the Bell Tolls".to_string()
    };
    print_put(&mut dao, &book0);

    dao.get(&book0.book_id);

    // let names = dao.load_names();
    // println!("size {} ", names.len());
    // for name in names {
    //     println!("author {}", name.author);
    // }
}

fn print_put(dao: &mut MyDao, book: &BookEntry) -> () {
    match dao.put(book) {
        Ok(_) => {
            println!("Book {:#?} was added", book.title);
        }
        Err(err) => {
            println!("Could not insert book {:#?} because of {:#?}", book.title, err);
        }
    }
}

// fn dynamo_list_tables_tests(dynamodb: &mut DynamoDBHelper) -> Result<(), DynamoDBError> {
//     let response = try!(dynamodb.list_tables());
//     println!("{:#?}", response);
//     Ok(())
// }
