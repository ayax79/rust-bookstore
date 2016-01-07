use uuid::Uuid;

#[derive(Debug)]
pub struct BookEntry {
    pub book_id: Uuid,
    pub author: String,
    pub title: String
}
