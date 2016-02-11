use uuid::Uuid;

#[derive(Debug,PartialEq,Eq,Hash,RustcEncodable,RustcDecodable)]
pub struct Book {
    pub book_id: Uuid,
    pub author: String,
    pub title: String
}
