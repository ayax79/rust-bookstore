extern crate rusoto;
use self::rusoto::regions::*;
use self::rusoto::credentials::*;
use self::rusoto::dynamodb::{DynamoDBHelper, CreateTableInput, DynamoDBError};
use self::rusoto::dynamodb::{AttributeDefinition, KeySchemaElement};

pub static REGION:&'static Region = &Region::UsWest2;
// pub static CREDS:&'static AWSCredentialsProvider = &DefaultAWSCredentialsProviderChain::new();

pub static BOOKS_TABLE:&'static str = "books";

fn create_credential_provider() -> DefaultAWSCredentialsProviderChain {
    DefaultAWSCredentialsProviderChain::new()
}

fn is_not_exists_err(s: &DynamoDBError) -> bool {
    // seems fields in DbError are currently not public
    // s.contains("ResourceNotFoundException")
    true
}

fn create_book_table(dynamodb: &mut DynamoDBHelper) -> Result<(), DynamoDBError> {
    let input = CreateTableInput::new()
                        .with_name(BOOKS_TABLE)
                        .with_write_capacity(1)
                        .with_read_capacity(1)
                        .with_attributes(attributes!("book_id" => "S"))
                        .with_key_schema(key_schema!("book_id" => "HASH"));

    let result = try!(dynamodb.create_table(&input));
    Ok(())
}

pub fn create_db_helper<'a>() -> DynamoDBHelper<'a> {
    let mut dynamodb = DynamoDBHelper::new(create_credential_provider(), REGION);

    match dynamodb.describe_table("books") {
        Ok(_) => {
            println!("books table exists, continuing.");
        }
        Err(ref err) if is_not_exists_err(err) => {
            println!("An error occurred ${:#?}", err);
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
    return dynamodb;
}
