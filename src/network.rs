use ipnetwork::IpNetwork;
use pnet::datalink::{self, NetworkInterface};
use crate::settings::Settings;
use std::net::SocketAddr;

const DEFAULT_PORT: u32 = 8080;
const DEFAULT_HOST: &'static str = "127.0.0.1";

/// Information about the server this microservice is hosted on
#[derive(Debug)]
pub struct NetworkInfo {
    pub ip_address: Option<String>,
}

impl NetworkInfo {
    pub fn new() -> Self {
        // This isn't perfect. Right now we just try to find the first ip4 non-loopback ip address
        let ip_address = datalink::interfaces()
            .into_iter()
            .find(NetworkInfo::is_valid_interface) // find a device that isn't a loopback device
            .and_then(|interface| {
                interface
                    .ips
                    .into_iter()
                    .find(|ip| ip.is_ipv4()) // find the ipv4 (skip ipv6)
                    .and_then(|ipnetwork| match ipnetwork {
                        IpNetwork::V4(ip4) => Some(ip4),
                        _ => None,
                    })
            })
            .map(|ip| ip.ip().to_string());

        NetworkInfo { ip_address }
    }

    pub fn build_server_socket_info(&self, settings: &Settings) -> SocketInfo {
        let ip_address = settings
            .server_address
            .to_owned()
            .or(self.ip_address.to_owned())
            .unwrap_or(DEFAULT_HOST.to_string());
        let port = settings.server_port.unwrap_or(DEFAULT_PORT);
        let full_addr = format!("{}:{}", ip_address, port);
        let socket_addr = full_addr.parse().unwrap();

        SocketInfo {
            ip_address,
            port,
            socket_addr,
        }
    }

    fn has_ip4(intf: &NetworkInterface) -> bool {
        intf.ips.iter().find(|i| i.is_ipv4()).is_some()
    }

    fn is_valid_interface(interface: &NetworkInterface) -> bool {
        interface.is_up() && !interface.is_loopback() && NetworkInfo::has_ip4(interface)
    }
}

/// Generic information about a socket
#[derive(Debug)]
pub struct SocketInfo {
    pub ip_address: String,
    pub port: u32,
    pub socket_addr: SocketAddr,
}

impl SocketInfo {
    #[allow(dead_code)] // i want to keep this around for now
    pub fn base_url(&self) -> String {
        let url = format!("http://{}:{}", self.ip_address, self.port);
        url.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ip_address() {
        let ni = NetworkInfo::new();
        let ip = ni.ip_address;
        assert!(ip.is_some());
        println!("ip network {:?}", ip.unwrap());
    }
}
