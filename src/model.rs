use uuid::Uuid;

#[derive(Debug,PartialEq,Eq,Hash)]
pub struct BookEntry {
    pub book_id: Uuid,
    pub author: String,
    pub title: String
}
