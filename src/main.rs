#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate tokio_core;
extern crate hyper;
extern crate futures;
extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate uuid;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate config;
extern crate rust_eureka;
extern crate pnet;
extern crate ipnetwork;

#[macro_use]
mod dynamo_utils;
mod model;
mod dao;
mod errors;
mod request;
mod service;
mod settings;
mod network;
mod eureka;

use dynamo_utils::initialize_db;
use settings::Settings;
use network::NetworkInfo;
use hyper::server::Server;
use hyper::service::service_fn;
use service::book_service;
use tokio_core::reactor::Core;
use eureka::EurekaHandler;
use futures::Future;

fn initialize(settings: &Settings) {
    let network_info = NetworkInfo::new();
    let socket_info = network_info.build_server_socket_info(settings);

    initialize_db();
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let eureka_handler = EurekaHandler::new(&handle, settings.clone(), &socket_info);
    for reg_future in eureka_handler.register() {
        let result = core.run(reg_future);
        info!("Eureka registration: {:?}", result);
    }

    let server = Server::bind(&socket_info.socket_addr)
        .serve(|| service_fn(book_service))
        .map_err(|err| eprintln!("server error: {}", err));

    println!("Starting BookService on {}", &socket_info.port);

    hyper::rt::run(server)

//    let server = Server::bind(&socket_info.socket_addr, || Ok(BookService)).unwrap();

//    let result = server.run();
//    debug!("server result {:?} ", &result)
}

fn main() {
    let result = env_logger::init();
    debug!("Log initialization: {:?}", &result);

    match Settings::new() {
        Ok(settings) => {
            initialize(&settings);
        },
        Err(e) => {
            panic!("could not initialize configuration: {}", e)
        }
    }

}