mod dao;
mod model;

use aws_config::{meta::region::RegionProviderChain, Region, SdkConfig};
use aws_sdk_dynamodb::{meta::PKG_VERSION, Client, Error as DynamoError};
use dao::BookEntryDao;
use std::error::Error;

use anyhow::anyhow;
use model::BookEntry;
use tracing::debug;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + 'static>> {
    tracing_subscriber::fmt()
        // this allows for RUST_LOG env_logger style env parameters options to be specified.
        // more can be found on this at https://docs.rs/env_logger/latest/env_logger/
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let config = make_config().await?;
    let client = Client::new(&config);
    let dao = BookEntryDao::new(client);

    let book0 = BookEntry {
        book_id: Uuid::new_v4(),
        author: "Ernest Hemmingway".to_string(),
        title: "For Whom the Bell Tolls".to_string(),
    };
    print_put(&dao, &book0).await;

    let book_result0 = dao
        .get(&book0.book_id)
        .await?
        .ok_or(anyhow!("No bookd returned for {}", book0.book_id))?;
    println!("result: {:#?}", book_result0);
    assert!(book0 == book_result0);

    Ok(())
}

async fn print_put(dao: &BookEntryDao, book: &BookEntry) {
    match dao.put(book).await {
        Ok(_) => {
            println!("Book {:#?} was added", book.title);
        }
        Err(err) => {
            println!(
                "Could not insert book {:#?} because of {:#?}",
                book.title, err
            );
        }
    }
}

pub fn make_region_provider() -> RegionProviderChain {
    RegionProviderChain::first_try(std::env::var("AWS_REGION").ok().map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"))
}

pub async fn make_config() -> Result<SdkConfig, DynamoError> {
    let region_provider = make_region_provider();

    debug!("DynamoDB client version: {}", PKG_VERSION);
    debug!(
        "Region:                  {}",
        region_provider.region().await.unwrap().as_ref()
    );

    Ok(aws_config::from_env().region(region_provider).load().await)
}
