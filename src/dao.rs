// mod cassandra_utils;
extern crate cassandra;

use self::cassandra::*;
use cassandra_utils::create_cluster;
use cassandra_utils::with_session;
use model::CommentEntry;

static COMMENTS_QUERY: &'static str = "SELECT object_type, object_id, comment_date, user_id, comment_id, comment_text, links FROM comments.comments;";

fn read_entry(row: Row) -> CommentEntry {
    CommentEntry {
        object_type: row.get_column_by_name("object_type")
                        .get_string()
                        .unwrap(),
        object_id: row.get_column_by_name("object_id")
                        .get_string()
                        .unwrap()
    }
}

pub struct MyDao { cluster: Box<Cluster> }

impl MyDao {

    pub fn new() -> MyDao {
        MyDao { cluster: Box::new(create_cluster()) }
    }

    pub fn load_names(&mut self) ->Vec<CommentEntry>  {
        with_session(self.cluster.as_mut(), |session| {
            session.execute(COMMENTS_QUERY, 0)
                    .wait()
                    .unwrap()
                    .iter()
                    .map(read_entry)
                    .collect()
        })
    }

}
