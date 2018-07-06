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
use hyper::server::Http;
use service::BookService;
use tokio_core::reactor::Core;
use eureka::EurekaHandler;

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

    let server = Http::new().bind(&socket_info.socket_addr, || Ok(BookService)).unwrap();

    println!("Starting BookService on {}", &socket_info.port);
    let result = server.run();
    debug!("server result {:?} ", &result)
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