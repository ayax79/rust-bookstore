use crate::model::BookEntry;
use anyhow::Result as AnyhowResult;
use aws_sdk_dynamodb::client::Client;
use aws_sdk_dynamodb::types::{
    AttributeValue, BillingMode, KeySchemaElement, KeyType, ProvisionedThroughput,
};
use uuid::Uuid;

pub const BOOKS_TABLE: &str = "books";

pub struct BookEntryDao {
    client: Client,
}

impl BookEntryDao {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn put(&self, entry: &BookEntry) -> AnyhowResult<()> {
        self.client
            .put_item()
            .table_name(BOOKS_TABLE)
            .item(
                "book_id",
                AttributeValue::S(format!("{}", entry.book_id.urn())),
            )
            .item("author", AttributeValue::S(entry.author.clone()))
            .item("title", AttributeValue::S(entry.title.clone()))
            .send()
            .await
            .map_err(anyhow::Error::new)
            .map(|_| ())
    }

    pub async fn get(&self, uuid: &Uuid) -> AnyhowResult<Option<BookEntry>> {
        self.client
            .get_item()
            .table_name(BOOKS_TABLE)
            .key("book_id", AttributeValue::S(format!("{}", uuid.urn())))
            .send()
            .await
            .map_err(anyhow::Error::new)
            .map(|output| output.item)
            .and_then(|item| item.map(BookEntry::try_from).transpose())
    }

    // todo - use this
    #[allow(dead_code)]
    async fn create_book_table(&self) -> AnyhowResult<()> {
        self.client
            .create_table()
            .table_name(BOOKS_TABLE)
            .billing_mode(BillingMode::Provisioned)
            .provisioned_throughput(
                ProvisionedThroughput::builder()
                    .read_capacity_units(5)
                    .write_capacity_units(5)
                    .build()?,
            )
            .key_schema(
                KeySchemaElement::builder()
                    .attribute_name("book_id")
                    .key_type(KeyType::Hash)
                    .build()?,
            )
            .send()
            .await
            .map_err(anyhow::Error::new)
            .map(|_| ())
    }
}
