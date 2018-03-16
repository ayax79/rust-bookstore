use std::collections::HashMap;
use std::thread;
use futures::{self, Future};
use futures::stream::{self, Stream};
use uuid::Uuid;
use rusoto_dynamodb::*;

use errors::BookServiceError;

use dynamo_utils::{BOOKS_TABLE, get_uuid_from_attribute, get_str_from_attribute};
use model::Book;



pub struct BookDao;

impl BookDao {
    pub fn new() -> BookDao {
        BookDao
    }

    pub fn put(&mut self, entry: &Book) -> Box<Future<Item=(), Error=BookServiceError>> {
        let item_map = item_map!(
            "book_id".to_string() => val!(S => entry.book_id.hyphenated().to_string()),
            "author".to_string() => val!(S => entry.author),
            "title".to_string() => val!(S => entry.title));

        let mut request = PutItemInput::default();
        request.item = item_map;
        request.table_name = BOOKS_TABLE.to_string();

        let client = build_db_client!();
        let result = client.put_item(&request)
            .map(|_| ())
            .map_err(|err| BookServiceError::BookCreateError(err));

        Box::new(futures::done(result))
    }


    pub fn get(&mut self, uuid: &Uuid) -> Box<Future<Item=Book, Error=BookServiceError>> {
        let mut request = GetItemInput::default();
        request.key = BookDao::create_key(uuid);
        request.table_name = BOOKS_TABLE.to_string();

        let client = build_db_client!();

        let result = client.get_item(&request)
            .and_then(|response| BookDao::read_entry(&response.item))
            .map_err(|err| BookServiceError::BookGetError(err));

        Box::new(futures::done(result))
    }


//    pub fn delete(&mut self, uuid: &Uuid) -> Result<(), DeleteItemError> {
//        let key = BookDao::create_key(uuid);
//        let mut request = DeleteRequest::default();
//        request.key = key;
//        // request.TableName = BOOKS_TABLE.to_string(); not yet implemented
//
//        // IT doesn't appear this is fully implemented yet.
//        Ok(())
//    }

    fn read_entry(item_map: &Option<HashMap<String, AttributeValue>>) -> Result<Book, GetItemError> {
        item_map
            .as_ref()
            .map(|item_map| {
                Book {
                    book_id: get_uuid_from_attribute(item_map.get("book_id").unwrap()).unwrap(),
                    author: get_str_from_attribute(item_map.get("author").unwrap()).unwrap().to_string(),
                    title: get_str_from_attribute(item_map.get("title").unwrap()).unwrap().to_string(),
                }
            })
            .ok_or(GetItemError::Unknown("Unknown error occurred".to_owned()))
    }

    fn create_key(uuid: &Uuid) -> HashMap<String, AttributeValue> {
        let mut key = HashMap::new();
        key.insert("book_id".to_string(), val!(S => uuid.hyphenated().to_string()));
        key
    }
}
