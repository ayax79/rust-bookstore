extern crate cassandra;

use self::cassandra::*;

static COMMENTS_QUERY: &'static str = "SELECT object_type, object_id, comment_date, user_id, comment_id, comment_text, links FROM comments.comments;";
static CONTACT_POINTS: &'static str = "127.0.0.1";

pub fn create_cluster() -> Cluster {
    let mut cluster = Cluster::new();
    cluster
        .set_contact_points(CONTACT_POINTS).unwrap()
        .set_load_balance_round_robin().unwrap();
    return cluster;
}

fn with_session<F,R>(cluster: &mut Cluster, f: F) -> R where F: Fn(&Session) -> R {
    let session = cluster.connect().unwrap();
    let result = f(&session);
    session.close().wait().unwrap();
    return result;
}

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
