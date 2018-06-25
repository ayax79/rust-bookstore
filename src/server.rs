use hyper::server::{Http, Server};
use settings::Settings;
use network::NetworkInfo;
use std::net::SocketAddr;

const DEFAULT_PORT: u16 = 8080;
const DEFAULT_HOST: &'static str = "127.0.0.1";

pub fn build_socket(settings: &Settings, server_ip: &Option<String>) -> SocketAddr {
    let addr = settings.server_address.to_owned()
        .or(server_ip.to_owned())
        .unwrap_or(DEFAULT_HOST.to_string());
    let port = settings.server_port.unwrap_or(DEFAULT_PORT);
    let full_addr = format!("{}:{}", addr, port);
    full_addr.parse().unwrap()
}

#[macro_export]
macro_rules! build_server {
    ($settings:expr, $network_info:expr) => ({

    })
}