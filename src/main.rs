extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate uuid;
#[macro_use]
extern crate log;
extern crate base64;
extern crate config;
extern crate env_logger;
extern crate ipnetwork;
extern crate pnet;
extern crate redis;
#[macro_use]
extern crate serde_derive;
extern crate core;
extern crate serde;
extern crate serde_json;

mod dao;
mod errors;
mod model;
mod network;
mod request;
mod service;
mod settings;

use futures::Future;
use hyper::server::Server;
use hyper::service::service_fn;
use network::NetworkInfo;
use service::book_service;
use settings::Settings;

fn main() {
    let log_result = env_logger::init();
    debug!("Log initialization: {:?}", &log_result);

    match Settings::new() {
        Ok(settings) => {
            let network_info = NetworkInfo::new();
            let socket_info = network_info.build_server_socket_info(&settings);

            let server = Server::bind(&socket_info.socket_addr)
                .serve(|| service_fn(book_service))
                .map_err(|err| eprintln!("server error: {}", err));

            println!("Starting BookService on {}", &socket_info.socket_addr);

            hyper::rt::run(server);
        }
        Err(e) => eprintln!("Could not load settings {}", e),
    }
    println!("BookStore service exiting")
}
