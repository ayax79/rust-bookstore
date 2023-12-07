use std::collections::HashMap;

use anyhow::{anyhow, Error as AnyhowError};
use aws_sdk_dynamodb::types::AttributeValue;
use uuid::Uuid;

pub const BOOK_ID: &str = "book_id";
pub const AUTHOR: &str = "author";
pub const TITLE: &str = "title";

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BookEntry {
    pub book_id: Uuid,
    pub author: String,
    pub title: String,
}

impl BookEntry {
    pub fn builder() -> BookEntryBuilder {
        BookEntryBuilder::default()
    }
}

#[derive(Default)]
pub struct BookEntryBuilder {
    book_id: Option<Uuid>,
    author: Option<String>,
    title: Option<String>,
}

impl BookEntryBuilder {
    pub fn with_book_id(self, book_id: Option<Uuid>) -> Self {
        Self { book_id, ..self }
    }

    pub fn with_author(self, author: Option<String>) -> Self {
        Self { author, ..self }
    }

    pub fn with_title(self, title: Option<String>) -> Self {
        Self { title, ..self }
    }

    pub fn build(self) -> Result<BookEntry, BookBuilderError> {
        Ok(BookEntry {
            book_id: self.book_id.ok_or(BookBuilderError::new(BOOK_ID))?,
            author: self.author.ok_or(BookBuilderError::new(AUTHOR))?,
            title: self.title.ok_or(BookBuilderError::new(TITLE))?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct BookBuilderError {
    field: String,
}

impl BookBuilderError {
    pub fn new(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
        }
    }
}

impl std::fmt::Display for BookBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Field {} is required: ", self.field)
    }
}

impl std::error::Error for BookBuilderError {}

impl TryFrom<HashMap<String, AttributeValue>> for BookEntry {
    type Error = AnyhowError;

    fn try_from(value: HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        let maybe_book_id = value
            .get(BOOK_ID)
            .map(|v| {
                v.as_s()
                    .map_err(|_| anyhow!("book_id was not a string"))
                    .and_then(|s| Uuid::parse_str(s).map_err(anyhow::Error::new))
            })
            .transpose()?;

        let maybe_author = value
            .get(AUTHOR)
            .map(|v| {
                v.as_s()
                    .map_err(|_| anyhow!("author was not a string"))
                    .cloned()
            })
            .transpose()?;

        let maybe_title = value
            .get(TITLE)
            .map(|v| {
                v.as_s()
                    .map_err(|_| anyhow!("title was not a string"))
                    .cloned()
            })
            .transpose()?;

        BookEntry::builder()
            .with_book_id(maybe_book_id)
            .with_author(maybe_author)
            .with_title(maybe_title)
            .build()
            .map_err(AnyhowError::new)
    }
}
