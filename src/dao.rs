use model::Book;
use rusoto::dynamodb::*;
use dynamo_utils::{create_dynamo_client, BOOKS_TABLE, get_uuid_from_attribute, get_str_from_attribute};
use uuid::Uuid;

pub struct BookDao;

impl BookDao {
    pub fn new() -> BookDao {
        BookDao
    }

    pub fn put(&mut self, entry: &Book) -> Result<(), PutItemError> {
        let item_map = item_map!(
            "book_id".to_string() => val!(S => entry.book_id.hyphenated().to_string()),
            "author".to_string() => val!(S => entry.author),
            "title".to_string() => val!(S => entry.title));

        let mut request = PutItemInput::default();
        request.item = item_map;
        request.table_name = BOOKS_TABLE.to_string();

        try!(create_dynamo_client().put_item(&request));
        Ok(())
    }

    pub fn get(&mut self, uuid: &Uuid) -> Result<Option<Book>, GetItemError> {
        let mut request = GetItemInput::default();
        request.key = BookDao::create_key(uuid);
        request.table_name = BOOKS_TABLE.to_string();

        match create_dynamo_client().get_item(&request) {
            Ok(response) => {
                Ok(BookDao::read_entry(&response.item))
            }
            Err(err) => Err(err)
        }
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

    fn read_entry(item_map: &Option<AttributeMap>) -> Option<Book> {
        item_map
            .as_ref()
            .map(|item_map| {
                Book {
                    book_id: get_uuid_from_attribute(item_map.get("book_id").unwrap()).unwrap(),
                    author: get_str_from_attribute(item_map.get("author").unwrap()).unwrap().to_string(),
                    title: get_str_from_attribute(item_map.get("title").unwrap()).unwrap().to_string()
                }
            })
    }

    fn create_key(uuid: &Uuid) -> Key {
        let mut key = Key::default();
        key.insert("book_id".to_string(), val!(S => uuid.hyphenated().to_string()));
        key
    }
}
