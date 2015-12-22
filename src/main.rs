extern crate cassandra;
use cassandra::*;
mod dao;
use dao::QUERY;
use dao::COL_NAME;
use dao::with_session;

fn main() {
    let selector = |session: &Session| {
        let result = session.execute(QUERY, 0).wait().unwrap();
        println!("{}", result);
        for row in result.iter() {
            println!("ks name = {}", row.get_column_by_name(COL_NAME));
        }
        return "hi";
    };
    let result = with_session(selector);
    println!("result {}", result);
}
