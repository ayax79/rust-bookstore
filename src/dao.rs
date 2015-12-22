extern crate cassandra;

use self::cassandra::*;

pub static QUERY: &'static str = "SELECT keyspace_name FROM system.schema_keyspaces;";
pub static COL_NAME: &'static str = "keyspace_name";
static CONTACT_POINTS: &'static str = "127.0.0.1";

pub fn create_cluster() -> Cluster {
    let mut cluster = Cluster::new();
    cluster
        .set_contact_points(CONTACT_POINTS).unwrap()
        .set_load_balance_round_robin().unwrap();
    return cluster;
}

pub fn with_session<F,R>(f: F) -> R where F: Fn(&Session) -> R {
    let mut cluster = create_cluster();
    let session = cluster.connect().unwrap();
    let result = f(&session);
    session.close().wait().unwrap();
    return result;
}
