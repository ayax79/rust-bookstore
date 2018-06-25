#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

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
#[macro_use]
mod server;

use dynamo_utils::initialize_db;
use settings::Settings;
use network::NetworkInfo;
use hyper::server::{Http, Server};
use server::build_socket;
use service::BookService;

fn initialize(settings: &Settings) {
    let network_info = NetworkInfo::new();
    let server_ip = network_info.map(|ni| ni.ip_address);

    initialize_db();

    let addr = build_socket(settings, &server_ip);
    let server = Http::new().bind(&addr, || Ok(BookService)).unwrap();

    println!("Starting BookService on {}", addr);
    server.run();
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