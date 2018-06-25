use pnet::datalink::{self, NetworkInterface};
use ipnetwork::IpNetwork;

#[derive(Debug)]
pub struct NetworkInfo {
    pub ip_address: String
}

impl NetworkInfo {
    pub fn new() -> Option<Self> {
        // This isn't perfect. Right now we just try to find the first ip4 non-loopback ip address
        datalink::interfaces()
            .into_iter()
            .find(NetworkInfo::is_valid_interface)     // find a device that isn't a loopback device
            .and_then(|interface| {
                interface.ips.into_iter()
                    .find(|ip| ip.is_ipv4())     // find the ipv4 (skip ipv6)
                    .and_then(|ipnetwork| {
                        match ipnetwork {
                            IpNetwork::V4(ip4) => Some(ip4),
                            _ => None
                        }
                    })
            })
            .map(|ip| {
                NetworkInfo {
                    ip_address: ip.ip().to_string()
                }
            })
    }

    fn has_ip4(intf: &NetworkInterface) -> bool {
        intf.ips.iter().find(|i| i.is_ipv4()).is_some()
    }

    fn is_valid_interface(interface: &NetworkInterface) -> bool {

        interface.is_up() &&
            !interface.is_loopback() &&
            NetworkInfo::has_ip4(interface)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ip_address() {
        let ni = NetworkInfo::new();
        assert!(ni.is_some());
        println!("ip network {:?}", ni.unwrap());
    }
}