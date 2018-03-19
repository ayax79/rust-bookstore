use uuid::Uuid;
use serde::{Serializer, Deserializer};
use serde::de::{self, Visitor};
use serde_json;
use std::fmt;
use errors::BookServiceError;

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Book {
    #[serde(serialize_with = "serialize_uuid", deserialize_with = "deserialize_uuid")]
    pub book_id: Uuid,
    pub author: String,
    pub title: String,
}

impl Book {
    pub fn new(book_id: Uuid, author: &str, title: &str) -> Self {
        Book {
            book_id: book_id,
            author: author.to_owned(),
            title: title.to_owned(),
        }
    }

    pub fn from_slice(slice: &[u8]) -> Result<Book, BookServiceError> {
        serde_json::from_slice(slice)
            .map_err(BookServiceError::BookParseError)
    }

    pub fn to_vec(&self) -> Result<Vec<u8>, BookServiceError> {
        serde_json::to_vec(self)
            .map_err(BookServiceError::BookSerializationError)
    }
}


fn serialize_uuid<S>(uuid: &Uuid, s: S) -> Result<S::Ok, S::Error> where S: Serializer {
    let uuid_string = uuid.hyphenated().to_string();
    s.serialize_str(uuid_string.as_ref())
}

fn deserialize_uuid<'de, D>(d: D) -> Result<Uuid, D::Error> where D: Deserializer<'de> {
    struct UuidVisitor;


    impl<'de> Visitor<'de> for UuidVisitor {
        type Value = Uuid;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("A valid uuid string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where
            E: de::Error {
            println!("received: {:?}", v);
            Uuid::parse_str(v).map_err(de::Error::custom)
        }
    }


    d.deserialize_str(UuidVisitor)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize() {
        let book = Book::new(Uuid::new_v4(), "Ernest Hemmingway", "For Whom the Bell Tolls");
        let book_result = serde_json::to_string(&book);
        assert!(book_result.is_ok());
        println!("book string {}", &book_result.unwrap());
    }

    #[test]
    fn test_deserialize() {
        let json = "{\"book_id\":\"87b17841-c677-4451-8bb9-64355b59c585\",\"author\":\"Ernest Hemmingway\",\"title\":\"For Whom the Bell Tolls\"}";
        let book_result = serde_json::from_str(json);
        assert!(book_result.is_ok());
        let book: Book = book_result.unwrap();
        assert_eq!("87b17841-c677-4451-8bb9-64355b59c585", book.book_id.hyphenated().to_string());
        assert_eq!("Ernest Hemmingway", book.author);
        assert_eq!("For Whom the Bell Tolls", book.title);
    }
}




