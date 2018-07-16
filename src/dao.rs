use uuid::Uuid;

use errors::BookServiceError;

use redis::{self, Commands, Client, PipelineCommands};
use model::Book;
use settings::Settings;
use std::collections::HashMap;

const AUTHOR: &'static str = "author";
const TITLE: &'static str = "title";

#[derive(Debug, Clone)]
pub struct BookDao {
    client: Client
}

impl BookDao {
    pub fn new(settings: &Settings) -> Result<BookDao, BookServiceError> {
        Client::open(settings.redis_url.as_ref())
            .map_err(|e| BookServiceError::DaoInitializationError(e))
            .map(|client| {
                BookDao {
                    client
                }
            })
    }

    pub fn put(&self, entry: &Book) -> Result<(), BookServiceError> {
        self.client.get_connection()
            .and_then(|conn| {
                let key = id_key(&entry.book_id);
                redis::pipe()
                    .atomic()
                    .hset(key.to_owned(), AUTHOR, entry.author.to_owned())
                    .hset(key.to_owned(), TITLE, entry.title.to_owned())
                    .query(&conn)
            })
            .map_err(|re| BookServiceError::BookCreateError(re))
    }

     pub fn get(&self, uuid: &Uuid) -> Result<Book, BookServiceError> {
         let key = id_key(uuid);
         self.client.get_connection()
             .and_then(|conn| conn.hgetall(key.to_owned()))
             .map_err(|e| BookServiceError::BookGetError(e))
             .and_then(|ref hm: HashMap<String, String>| book_from_map(key.as_ref(), hm))
     }

}

fn book_from_map(key: &str, hm: &HashMap<String, String>) -> Result<Book, BookServiceError> {
    uuid_from_key(key)
        .and_then(|book_id| {
            let author = hm.get(AUTHOR).ok_or(BookServiceError::MissingFieldError(AUTHOR.to_string()))?;
            let title = hm.get(TITLE).ok_or(BookServiceError::MissingFieldError(TITLE.to_string()))?;
            Ok(Book{
                book_id,
                author: author.to_owned(),
                title: title.to_owned(),
            })
        })
}

fn id_key(uuid: &Uuid) -> String {
    "BOOK-".to_string() +
        uuid.hyphenated().to_string().as_ref()
}

fn uuid_from_key(key: &str) -> Result<Uuid, BookServiceError> {
    let minus_prefix = &key[4..];
    Uuid::parse_str(minus_prefix)
        .map_err(BookServiceError::from)
}