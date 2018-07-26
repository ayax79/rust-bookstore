use uuid::Uuid;

use std::convert::AsRef;
use std::collections::HashMap;
use redis::{self, Commands, Client, PipelineCommands};
use base64;
use model::Book;
use errors::BookServiceError;
use settings::Settings;

const AUTHOR: &'static str = "author";
const TITLE: &'static str = "title";

#[derive(Debug, Clone)]
pub struct BookDao {
    client: Client
}

impl BookDao {
    pub fn new(settings: &Settings) -> Result<BookDao, BookServiceError> {
        redis_url(settings)
            .and_then(|url| {
                Client::open(url.as_ref())
                    .map_err(|e| {
                        eprintln!("Could not open redis connection! {} ", &e);
                        BookServiceError::DaoInitializationError(e)
                    })
            })
            .map(|client| {
                BookDao {
                    client
                }
            })
    }

    pub fn put(&self, entry: &Book) -> Result<(), BookServiceError> {
        println!("put for book {:?}", &entry);
        self.client.get_connection()
            .and_then(|conn| {
                let key = id_key(&entry.book_id);
                redis::pipe()
                    .atomic()
                    .hset(key.to_owned(), AUTHOR, entry.author.to_owned())
                    .hset(key.to_owned(), TITLE, entry.title.to_owned())
                    .query(&conn)
            })
            .map_err(|re| {
                eprintln!("Failed to put book {:?}", &re);
                BookServiceError::BookCreateError(re)
            })
    }

    pub fn get(&self, uuid: &Uuid) -> Result<Book, BookServiceError> {
        let key = id_key(uuid);
        self.client.get_connection()
            .and_then(|conn| conn.hgetall(key.to_owned()))
            .map_err(|e| {
                eprintln!("Error Getting book {}", &e);
                BookServiceError::BookGetError(e)
            })
            .and_then(|ref hm: HashMap<String, String>| book_from_map(key.as_ref(), hm))
    }
}

fn book_from_map(key: &str, hm: &HashMap<String, String>) -> Result<Book, BookServiceError> {
    uuid_from_key(key)
        .and_then(|book_id| {
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
    "BOOK-".to_string() +
        uuid.hyphenated().to_string().as_ref()
}

fn uuid_from_key(key: &str) -> Result<Uuid, BookServiceError> {
    let minus_prefix = &key[5..];
    Uuid::parse_str(minus_prefix)
        .map_err(|e| {
            eprintln!("Unable to parse UUID from key: {}", key);
            BookServiceError::from(e)
        })
}

fn redis_url(settings: &Settings) -> Result<String, BookServiceError> {
    settings.redis_password()
        .ok_or_else(|| {
            eprintln!("Redis password was not specified");
            BookServiceError::RedisPasswordError
        })
        .and_then(|password| {
            settings.redis_host()
                .map(|host| (host, password))
                .ok_or_else (|| {
                    eprintln!("Redis host was not specified");
                    BookServiceError::RedisHostError
                })
        })
        .and_then(|(host, password)| {
            settings.redis_port()
                .map(|port| (host, port, password))
                .ok_or_else(|| {
                    eprintln!("Redis port was not specified");
                    BookServiceError::RedisPortError
                })
        })
        .map(|(host, port, password)| {
            format!("redis://:{}@{}:{}", password, host, port)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use config::Config;

    #[test]
    fn test_redit_url() {
        let mut config = Config::new();
        &config.set("redishost", "myredishost").unwrap()
            .set("redispassword", "bXlyZWRpc3Bhc3MK").unwrap()
            .set("redisport", "6363").unwrap();
        let test_settings = Settings::with_config(config.clone());

        let url = "redis://:myredispass@myredishost:6363";
        let result = redis_url(&test_settings).unwrap();
        assert_eq!(url.to_string(), result);
    }

    #[test]
    fn test_uuid_from_key() {
        let key = "BOOK-0bcd291d-b7c5-4390-965f-8a70707d22a5";
        let result = uuid_from_key(key);
        assert!(result.is_ok());
    }

}