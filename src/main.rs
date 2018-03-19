#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate hyper;
extern crate futures;
extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate uuid;


#[macro_use]
mod dynamo_utils;
mod model;
mod dao;
mod errors;
mod request;
mod service;

use dynamo_utils::initialize_db;



//#[get("/book/<book_id_wrapper>")]
//fn get_book(book_id_wrapper: UuidWrapper) -> Option<Json<Book>> {
//    let UuidWrapper(book_id) = book_id_wrapper;
//    let mut dao = BookDao::new();
//    dao.get(&book_id).map(|b| Json(b)).ok()
//}
//


fn main() {
    initialize_db();


//    let router = router!(
//        get_book: get "/books/:book_id" => get_book,
//        get_books: get "/books" => book_list,
//        put_book: put "/books/:book_id" => create_book
//    );
//
//    fn book_list(_: &mut Request) -> IronResult<Response> {
//        let book0 = Book {
//            book_id: Uuid::new_v4(),
//            author: "Ernest Hemmingway".to_string(),
//            title: "For Whom the Bell Tolls".to_string()
//        };
//        let payload = serde_json::to_string(&book0).unwrap();
//        Ok(Response::with((status::Ok, payload)))
//    }
//
//    fn get_book(request: &mut Request) -> IronResult<Response> {
//        let mut dao = BookDao::new();
//        let ref book_id_string = request.extensions.get::<Router>().unwrap().find("book_id").unwrap();
//        let book_id = Uuid::parse_str(book_id_string).unwrap();
//        let book:Book = dao.get(&book_id).unwrap().unwrap();
//        let payload = serde_json::to_string(&book).unwrap();
//        Ok(Response::with((status::Ok, payload)))
//    }
//
//    fn create_book(request: &mut Request) -> IronResult<Response> {
//        let mut dao = BookDao::new();
//        let mut payload = String::new();
//        request.body.read_to_string(&mut payload).unwrap();
//        let book: Book = serde_json::from_str(&payload).unwrap();
//        print_put(&mut dao, &book);
//        Ok(Response::with((status::Ok, payload)))
//    }
//
//    println!("Starting bookstore server on port 8080");
//    Iron::new(router).http("0.0.0.0:8080").unwrap();
}

//fn print_put(dao: &mut BookDao, book: &Book) -> () {
//    match dao.put(book) {
//        Ok(_) => {
//            println!("Book {:#?} was added", book.title);
//        }
//        Err(err) => {
//            println!("Could not insert book {:#?} because of {:#?}", book.title, err);
//        }
//    }
//}