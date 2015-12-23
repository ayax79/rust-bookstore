extern crate cassandra;

use self::cassandra::*;
use cassandra_utils::create_cluster;
use cassandra_utils::with_session;
use model::BookEntry;

static BOOKS_QUERY: &'static str = "SELECT * FROM books.books";

fn read_entry(row: Row) -> BookEntry {
    BookEntry {
        book_id: row.get_column_by_name("book_id")
                        .get_string()
                        .unwrap(),
        author: row.get_column_by_name("author")
                        .get_string()
                        .unwrap(),
        title: row.get_column_by_name("title")
                        .get_string()
                        .unwrap()
    }
}

pub struct MyDao { cluster: Box<Cluster> }

impl MyDao {

    pub fn new() -> MyDao {
        MyDao { cluster: Box::new(create_cluster()) }
    }

    pub fn load_names(&mut self) ->Vec<BookEntry>  {
        with_session(self.cluster.as_mut(), |session| {
            session.execute(BOOKS_QUERY, 0)
                    .wait()
                    .unwrap()
                    .iter()
                    .map(read_entry)
                    .collect()
        })
    }

}
