use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::dynamodb::*;
use uuid::Uuid;
use hyper::client::Client;
use std::str::from_utf8;
use rusoto::default_tls_client;

type SimpleDynamoClient = DynamoDbClient<DefaultCredentialsProvider, Client>;

pub static BOOKS_TABLE:&'static str = "books";


#[macro_export]
macro_rules! val {
	(B => $val:expr) => (
	    {
	    	let mut attr = AttributeValue::default();
	    	attr.b = Some($val);
	    	attr
	    }
	);
	(S => $val:expr) => (
	    {
			let mut attr = AttributeValue::default();
			attr.s = Some($val.to_string());
			attr
		}
	);
	(N => $val:expr) => (
	    {
	    	let mut attr = AttributeValue::default();
	    	attr.n = Some($val.to_string());
	    	attr
	    }
	);
}

#[macro_export]
macro_rules! attributes {
    ($($val:expr => $attr_type:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(AttributeDefinition { attribute_name: String::from($val), attribute_type: String::from($attr_type) });
            )*
            temp_vec
        }
    }
}

#[macro_export]
macro_rules! key_schema {
    ($($name:expr => $key_type:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(KeySchemaElement { key_type: String::from($key_type), attribute_name: String::from($name) });
            )*
            temp_vec
        }
    }
}

#[macro_export]
macro_rules! item_map {
    ($($map_key:expr => $map_value:expr),*) => {
        {
            let mut temp_map = PutItemInputAttributeMap::new();
            $(
                temp_map.insert(String::from($map_key), $map_value);
            )*
            temp_map
        }
    }
}

pub fn initialize_db() {
    let mut dynamodb = create_dynamo_client();
    let request = DescribeTableInput { table_name: "books".to_string() };
    match dynamodb.describe_table(&request) {
        Ok(_) => {
            println!("books table exists, continuing.");
        }
        Err(DescribeTableError::ResourceNotFound(msg)) => {
            println!("An error occurred ${:#?}", msg);
            println!("books table may not exist, creating");
            match create_book_table(&mut dynamodb) {
                Ok(_) => {
                    println!("successfully created books table")
                }
                Err(err) => {
                    println!("Could not create books table ${:#?}", err)
                }
            }
        }
        Err(err) => {
            println!("An error occurred ${:#?}", err);
        }
    }
}

pub fn get_str_from_attribute(attr: &AttributeValue) -> Option<&str> {
    match attr.b {
        None => (),
        Some(ref blob_attribute) => return Some(from_utf8(blob_attribute).unwrap()),
    }

    match attr.s {
        None => (),
        Some(ref string_attribute) => return Some(string_attribute),
    }

    match attr.n {
        None => (),
        Some(ref number_attribute) => return Some(number_attribute),
    }

    return None;
}

pub fn get_uuid_from_attribute(attr: &AttributeValue) -> Option<Uuid> {
    get_str_from_attribute(attr)
        .map(|uuid_string| Uuid::parse_str(uuid_string))
        .and_then(|uuid_result| uuid_result.ok())
}

pub fn create_dynamo_client() -> SimpleDynamoClient {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    DynamoDbClient::new(default_tls_client().unwrap(), credentials, Region::UsWest2)
}

fn create_book_table(client: &mut DynamoDbClient<DefaultCredentialsProvider, Client>) -> Result<(), CreateTableError> {
    let provisioning = ProvisionedThroughput {
        read_capacity_units: 1,
        write_capacity_units: 1
    };
    let input = CreateTableInput {
        attribute_definitions: attributes!("book_id" => "S"),
        key_schema: key_schema!("book_id" => "HASH"),
        provisioned_throughput: provisioning,
        table_name: "books".to_string(),
        local_secondary_indexes: None,
        global_secondary_indexes: None,
        stream_specification: None
    };
    try!(client.create_table(&input));
    Ok(())
}
