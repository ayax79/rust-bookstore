use uuid::Uuid;

use crate::errors::BookServiceError;
use crate::errors::DaoCause;
use crate::model::Book;
use crate::settings::Settings;
use r2d2_redis::redis::Commands;
use r2d2_redis::{r2d2, RedisConnectionManager};
use redis::{self, PipelineCommands};
use std::collections::HashMap;
use std::convert::AsRef;
use std::ops::Deref;

const AUTHOR: &'static str = "author";
const TITLE: &'static str = "title";

#[derive(Debug, Clone)]
pub struct BookDao {
    redis_pool: r2d2::Pool<RedisConnectionManager>,
}

impl BookDao {
    pub fn new(settings: &Settings) -> Result<BookDao, BookServiceError> {
        settings
            .redis_url()
            .and_then(|url| {
                RedisConnectionManager::new(url.as_ref()).map_err(|e| {
                    eprintln!("Could not create connection manager! {} ", &e);
                    BookServiceError::from(e)
                })
            })
            .and_then(|mgr| {
                r2d2::Pool::builder().build(mgr).map_err(|e| {
                    eprintln!("Could not create connection pool! {} ", &e);
                    BookServiceError::from(e)
                })
            })
            .map(|connection_mgr| BookDao {
                redis_pool: connection_mgr,
            })
    }

    pub fn put(&self, entry: &Book) -> Result<(), BookServiceError> {
        println!("put for book {:?}", &entry);
        self.redis_pool
            .get()
            .map_err(|e| {
                eprintln!("Failed to put book {:?}", &e);
                BookServiceError::BookCreateError(DaoCause::from(e))
            })
            .and_then(|conn| {
                let key = id_key(&entry.book_id);
                redis::pipe()
                    .atomic()
                    .hset(key.to_owned(), AUTHOR, entry.author.to_owned())
                    .hset(key.to_owned(), TITLE, entry.title.to_owned())
                    .query(conn.deref())
                    .map_err(|e| {
                        eprintln!("Failed to put book {:?}", &e);
                        BookServiceError::BookCreateError(DaoCause::from(e))
                    })
            })
    }

    pub fn get(&self, uuid: &Uuid) -> Result<Book, BookServiceError> {
        let key = id_key(uuid);
        self.redis_pool
            .get()
            .map_err(|e| {
                eprintln!("Error Getting book {}", &e);
                BookServiceError::BookGetError(DaoCause::from(e))
            })
            .and_then(|conn| {
                conn.hgetall(key.to_owned()).map_err(|e| {
                    eprintln!("Error Getting book {}", &e);
                    BookServiceError::BookGetError(DaoCause::from(e))
                })
            })
            .and_then(|ref hm: HashMap<String, String>| book_from_map(key.as_ref(), hm))
    }
}

fn book_from_map(key: &str, hm: &HashMap<String, String>) -> Result<Book, BookServiceError> {
    uuid_from_key(key).and_then(|book_id| {
        let author = hm.get(AUTHOR).ok_or_else(|| {
            eprintln!("Book entry for key {} does not contain field author", key);
            BookServiceError::MissingFieldError(AUTHOR.to_string())
        })?;
        let title = hm.get(TITLE).ok_or_else(|| {
            eprintln!("Book entry for key {} does not contain field title", key);
            BookServiceError::MissingFieldError(TITLE.to_string())
        })?;
        Ok(Book {
            book_id,
            author: author.to_owned(),
            title: title.to_owned(),
        })
    })
}

fn id_key(uuid: &Uuid) -> String {
    "BOOK-".to_string() + uuid.hyphenated().to_string().as_ref()
}

fn uuid_from_key(key: &str) -> Result<Uuid, BookServiceError> {
    let minus_prefix = &key[5..];
    Uuid::parse_str(minus_prefix).map_err(|e| {
        eprintln!("Unable to parse UUID from key: {}", key);
        BookServiceError::from(e)
    })
}

#[cfg(test)]
mod tests {
    use testcontainers;

    use self::testcontainers::*;
    use super::*;

    #[test]
    fn test_uuid_from_key() {
        let key = "BOOK-0bcd291d-b7c5-4390-965f-8a70707d22a5";
        let result = uuid_from_key(key);
        assert!(result.is_ok());
    }

    #[test]
    fn test_put_get() {
        let docker = clients::Cli::default();
        let node = docker.run(images::redis::Redis::default());
        let host_port = node.get_host_port(6379).unwrap();

        let settings = Settings::default()
            .with_redis_host("localhost")
            .with_redis_port(host_port);

        let key = "BOOK-0bcd291d-b7c5-4390-965f-8a70707d22a5";
        let book_id = uuid_from_key(key).unwrap();

        let book = Book::default()
            .with_book_id(&book_id)
            .with_author("Robert")
            .with_title("Jordan");

        let dao = BookDao::new(&settings).unwrap();

        let _ = dao.put(&book).unwrap();

        let result = dao.get(&book_id).unwrap();

        assert_eq!(book, result);
    }
}
