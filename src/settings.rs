use config::{ConfigError, Config, Environment};


#[derive(Debug,Clone)]
pub struct Settings {
    pub server_address: Option<String>,
    pub server_port: Option<u16>,
    pub eureka_url: Option<String>,
    pub hostname: Option<String>
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        Config::new()
            .merge(Environment::with_prefix("bookstore"))
            .map(|config| {
                let server_address = config.get("serveraddress").ok();
                let server_port = config.get("serverport").ok();
                let eureka_url = config.get("eureka_url").ok();
                let hostname = config.get("hostname").ok();
                Settings {
                    server_address,
                    server_port,
                    eureka_url,
                    hostname
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_server_address_default() {
        env::remove_var("BOOKSTORE_SERVERADDRESS");
        let settings = Settings::new().unwrap();
        assert!(settings.server_address.is_none())
    }

    #[test]
    fn test_server_address_overridden() {
        let addr = "0.0.0.0:80";
        env::set_var("BOOKSTORE_SERVER_ADDRESS", addr);
        assert_eq!("0.0.0.0:80", env::var("BOOKSTORE_SERVERADDRESS").unwrap());
        let settings = Settings::new().unwrap();
        assert_eq!("0.0.0.0:80", settings.server_address.unwrap());
    }

}

