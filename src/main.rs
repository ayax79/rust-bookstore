mod dao;
use dao::create_cluster;
use dao::load_names;

fn main() {
    let mut cluster = create_cluster();
    let names = load_names(&mut cluster);
    for name in names {
        println!("name {}", name);
    }
}
