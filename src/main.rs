#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate tokio_core;
extern crate hyper;
extern crate futures;
extern crate uuid;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate config;
extern crate pnet;
extern crate ipnetwork;
extern crate redis;
extern crate core;
#[macro_use]
extern crate lazy_static;

mod model;
mod dao;
mod errors;
mod request;
mod service;
mod settings;
mod network;

use settings::Settings;
use network::NetworkInfo;
use hyper::server::Server;
use hyper::service::service_fn;
use service::book_service;
use futures::Future;


lazy_static! {
    static ref SETTINGS: Settings = Settings::new().unwrap();
}


fn main() {
    let log_result = env_logger::init();
    debug!("Log initialization: {:?}", &log_result);

    let network_info = NetworkInfo::new();
    let socket_info = network_info.build_server_socket_info(&SETTINGS);

    let server = Server::bind(&socket_info.socket_addr)
        .serve(|| service_fn(|req| book_service(&SETTINGS, req)))
        .map_err(|err| eprintln!("server error: {}", err));

    println!("Starting BookService on {}", &socket_info.port);

    hyper::rt::run(server);


    println!("BookStore service exiting")
}

