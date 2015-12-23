// mod cassandra_utils;
extern crate cassandra;

use self::cassandra::*;
use cassandra_utils::create_cluster;
use cassandra_utils::with_session;

static COMMENTS_QUERY: &'static str = "SELECT object_type, object_id, comment_date, user_id, comment_id, comment_text, links FROM comments.comments;";

pub struct MyDao { cluster: Box<Cluster> }

impl MyDao {

    pub fn new() -> MyDao {
        MyDao { cluster: Box::new(create_cluster()) }
    }

    pub fn load_names(&mut self) -> Vec<String> {
        let selector = |session: &Session| {
            let mut names = vec![];
            let result = session.execute(COMMENTS_QUERY, 0).wait().unwrap();
            for row in result.iter() {
                let row_name = row.get_column_by_name("comment_text");
                println!("ks name = {}", row_name);
                names.push(row_name.get_string().unwrap());
            }
            return names;
        };
        let result = with_session(self.cluster.as_mut(), selector);
        return result;
    }
}
