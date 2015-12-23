mod dao;
mod model;
mod cassandra_utils;
use dao::MyDao;

fn main() {
    let mut dao = MyDao::new();
    let names = dao.load_names();
    for name in names {
        println!("name {}", name.object_type);
    }
}
