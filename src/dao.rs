use model::BookEntry;
use rusoto::dynamodb::{DynamoDBError, DynamoDBHelper, PutItemInputAttributeMap};
use rusoto::dynamodb::{AttributeValue, PutItemInput, Key, GetItemInput};
use dynamo_utils::{create_db_helper, BOOKS_TABLE};
use uuid::Uuid;
use std::collections::HashMap;

fn read_entry(item_map: HashMap<String, AttributeValue>) -> BookEntry {
    BookEntry {
        book_id: Uuid::new_v4(),
        author: "bar".to_string(),
        title: "baz".to_string()
    }
}

fn build_put_item_input(entry: &BookEntry) -> PutItemInput {
    let mut input = PutItemInput::default();
    input.Item = create_put_item_map(entry);
    input.TableName = BOOKS_TABLE.to_string();
    return input;
}

fn create_put_item_map(entry: &BookEntry) -> PutItemInputAttributeMap {
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
    request.Key = create_key(uuid);
    request.TableName = BOOKS_TABLE.to_string();
    return request;
}

pub struct MyDao<'a> { dynamodb: Box<DynamoDBHelper<'a> > }

impl <'a> MyDao<'a> {

    pub fn new() -> MyDao<'a>   {
        MyDao { dynamodb: Box::new(create_db_helper()) }
    }

    pub fn put(&mut self, entry: &BookEntry) -> Result<(), DynamoDBError> {
        let item = build_put_item_input(entry);
        try!(self.dynamodb.as_mut().put_item(&item));
        Ok(())
    }

    pub fn get(&mut self, uuid: &Uuid) -> Result<Option<BookEntry>, DynamoDBError> {
        let request = create_get_item_input(uuid);

        match (self.dynamodb.as_mut().get_item(&request)) {
            Ok(item) => {
                Ok(item.Item.map(|item_map| read_entry(item_map)))
            }
            Err(err) => Err(err)
        }
    }

}
