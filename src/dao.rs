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
                    .map_err(|e| BookServiceError::DaoInitializationError(e))
            })
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
    let minus_prefix = &key[4..];
    Uuid::parse_str(minus_prefix)
        .map_err(BookServiceError::from)
}


fn redis_url(settings: &Settings) -> Result<String, BookServiceError> {
    settings.redis_password()
        .ok_or(BookServiceError::RedisPasswordError)
        .and_then(|password| {
            base64::decode(password.as_bytes())
                .map_err(|_| BookServiceError::RedisPasswordError)
        })
        .and_then(|pass_as_bytes| {
            String::from_utf8(pass_as_bytes)
                .map(|s| s.trim().to_owned())
                .map_err(|_| BookServiceError::RedisPasswordError)
        })
        .and_then(|password| {
            settings.redis_host()
                .map(|host| (host, password))
                .ok_or(BookServiceError::RedisHostError)
        })
        .and_then(|(host, password)| {
            settings.redis_port()
                .map(|port| (host, port, password))
                .ok_or(BookServiceError::RedisPortError)
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
}