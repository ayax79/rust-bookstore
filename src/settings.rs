use std::convert::From;
use config::{Config, Environment};
use errors::BookServiceError;


#[derive(Debug,Clone)]
pub struct Settings {
    pub server_address: Option<String>,
    pub server_port: Option<u16>,
    pub redis_url: String,
    pub hostname: Option<String>
}

impl Settings {
    pub fn new() -> Result<Self, BookServiceError> {
        Config::new()
            .merge(Environment::with_prefix("bookstore"))
            .map_err(BookServiceError::from)
            .and_then(|config| {
                let server_address = config.get("serveraddress").ok();
                let server_port = config.get("serverport").ok();
                let redis_url = config.get("redis_url").map_err(BookServiceError::from)?;
                let hostname = config.get("hostname").ok();
                Ok(Settings {
                    server_address,
                    server_port,
                    redis_url,
                    hostname
                })
            })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    #[ignore] // todo - fix
    fn test_server_address_default() {
        env::remove_var("BOOKSTORE_SERVERADDRESS");
        let settings = Settings::new().unwrap();
        assert!(settings.server_address.is_none())
    }

    #[test]
    #[ignore] // todo - fix
    fn test_server_address_overridden() {
        let addr = "0.0.0.0:80";
        env::set_var("BOOKSTORE_SERVER_ADDRESS", addr);
        assert_eq!("0.0.0.0:80", env::var("BOOKSTORE_SERVERADDRESS").unwrap());
        let settings = Settings::new().unwrap();
        assert_eq!("0.0.0.0:80", settings.server_address.unwrap());
    }

}

