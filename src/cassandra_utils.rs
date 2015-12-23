extern crate cassandra;
use self::cassandra::*;

static CONTACT_POINTS: &'static str = "127.0.0.1";

static CREATE_KEYSPACE:&'static str = "CREATE KEYSPACE IF NOT EXISTS books WITH replication = { \'class\': \
                                      \'SimpleStrategy\', \'replication_factor\': \'3\' };";

static CREATE_TABLE:&'static str = "CREATE TABLE IF NOT EXISTS books.books (book_id UUID, title TEXT, author TEXT, \
                                    PRIMARY KEY (book_id));";

pub fn create_cluster() -> Cluster {
    let mut cluster = Cluster::new();
    cluster
        .set_contact_points(CONTACT_POINTS).unwrap()
        .set_load_balance_round_robin().unwrap();
    init(&mut cluster);
    cluster
}

pub fn with_session<F,R>(cluster: &mut Cluster, f: F) -> R where F: Fn(&Session) -> R {
    let session = cluster.connect().unwrap();
    let result = f(&session);
    session.close().wait().unwrap();
    result
}

fn init(cluster: &mut Cluster) {
    with_session(cluster, |session| {
        session.execute_statement(&Statement::new(CREATE_KEYSPACE,0));
        session.execute_statement(&Statement::new(CREATE_TABLE,0));
    });
}
