use model::Book;
use rusoto::dynamodb::{DynamoDBError, DynamoDBHelper, PutItemInputAttributeMap, DeleteRequest};
use rusoto::dynamodb::{AttributeValue, PutItemInput, Key, GetItemInput, get_str_from_attribute};
use dynamo_utils::{create_db_helper, BOOKS_TABLE, get_uuid_from_attribute};
use uuid::Uuid;
use std::collections::HashMap;

pub struct BookDao<'a> { dynamodb: Box<DynamoDBHelper<'a> > }

impl <'a> BookDao<'a> {

    pub fn new() -> BookDao<'a>   {
        BookDao { dynamodb: Box::new(create_db_helper()) }
    }

    pub fn put(&mut self, entry: &Book) -> Result<(), DynamoDBError> {
        let item = BookDao::build_put_item_input(entry);
        try!(self.dynamodb.as_mut().put_item(&item));
        Ok(())
    }

    pub fn get(&mut self, uuid: &Uuid) -> Result<Option<Book>, DynamoDBError> {
        let request = BookDao::create_get_item_input(uuid);

        match self.dynamodb.as_mut().get_item(&request) {
            Ok(item) => {
                Ok(item.Item.map(|item_map| BookDao::read_entry(item_map)))
            }
            Err(err) => Err(err)
        }
    }

    pub fn delete(&mut self, uuid: &Uuid) -> Result<(), DynamoDBError> {
        let request = BookDao::create_delete_request(uuid);
        // try!(self.dynamodb.as_mut().)

        // IT doesn't appear this is fully implemented yet.
        Ok(())
    }

    fn read_entry(item_map: HashMap<String, AttributeValue>) -> Book {
        Book {
            book_id: get_uuid_from_attribute(&item_map.get("book_id").unwrap()).unwrap(),
            author: get_str_from_attribute(&item_map.get("author").unwrap()).unwrap().to_string(),
            title: get_str_from_attribute(&item_map.get("title").unwrap()).unwrap().to_string()
        }
    }

    fn build_put_item_input(entry: &Book) -> PutItemInput {
        let mut input = PutItemInput::default();
        input.Item = BookDao::create_put_item_map(entry);
        input.TableName = BOOKS_TABLE.to_string();
        return input;
    }

    fn create_put_item_map(entry: &Book) -> PutItemInputAttributeMap {
        let mut item_map = PutItemInputAttributeMap::default();
        item_map.insert("book_id".to_string(), val!(S => entry.book_id.to_urn_string()));
        item_map.insert("author".to_string(), val!(S => entry.author));
        item_map.insert("title".to_string(), val!(S => entry.title));
        return item_map;
    }

    fn create_key(uuid: &Uuid) -> Key {
        let mut key = Key::default();
        key.insert("book_id".to_string(), val!(S => uuid.to_urn_string()));
        return key;
    }

    fn create_get_item_input(uuid: &Uuid) -> GetItemInput {
        let mut request = GetItemInput::default();
        request.Key = BookDao::create_key(uuid);
        request.TableName = BOOKS_TABLE.to_string();
        return request;
    }

    fn create_delete_request(uuid: &Uuid) -> DeleteRequest {
        let key = BookDao::create_key(uuid);
        let mut request = DeleteRequest::default();
        request.Key = key;
        // request.TableName = BOOKS_TABLE.to_string(); not yet implemented
        return request;
    }
}
