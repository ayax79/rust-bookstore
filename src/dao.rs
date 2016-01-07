use model::BookEntry;
use rusoto::dynamodb::{DynamoDBError, DynamoDBHelper, PutItemInputAttributeMap};
use rusoto::dynamodb::{AttributeValue, PutItemInput};
use dynamo_utils::create_db_helper;
use uuid::Uuid;

fn read_entry() -> BookEntry {
    BookEntry {
        book_id: Uuid::new_v4(),
        author: "bar".to_string(),
        title: "baz".to_string()
    }
}

fn build_put_item_input(entry: &BookEntry) -> PutItemInput {
    let mut input = PutItemInput::default();
    input.Item = create_item_map(entry);
    input.TableName = "books".to_string();
    return input;
}

fn create_item_map(entry: &BookEntry) -> PutItemInputAttributeMap {
    let mut item_map = PutItemInputAttributeMap::default();
    item_map.insert("book_id".to_string(), val!(S => entry.book_id));
    item_map.insert("author".to_string(), val!(S => entry.author));
    item_map.insert("title".to_string(), val!(S => entry.title));
    return item_map;
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

    // pub fn get(&mut self, uuid: &UUID) -> {
    //
    // }

}
