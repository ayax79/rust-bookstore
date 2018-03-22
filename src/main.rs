#![feature(conservative_impl_trait)]

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

#[macro_use]
mod dynamo_utils;
mod model;
mod dao;
mod errors;
mod request;
mod service;

use dynamo_utils::initialize_db;
use hyper::server::Http;
use service::BookService;

fn main() {
    env_logger::init();

    initialize_db();
    let add_str = "127.0.0.1:3000";
    let addr = add_str.parse().unwrap();
    info!("Starting BookService on {}", add_str);
    let server = Http::new().bind(&addr, || Ok(BookService)).unwrap();
    server.run().unwrap();
}