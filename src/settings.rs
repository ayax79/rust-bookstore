use config::{Config, Environment};
use errors::BookServiceError;
use std::convert::From;

#[derive(Debug, Clone)]
pub struct Settings {
    config: Config,
}

impl Settings {
    pub fn new() -> Result<Self, BookServiceError> {
        Config::new()
            .merge(Environment::with_prefix("bookstore"))
            .map_err(BookServiceError::from)
            .map(|config| Settings {
                config: config.to_owned(),
            })
    }

    #[allow(dead_code)] // useful for testing
    pub fn with_config(config: Config) -> Settings {
        Settings { config }
    }

    pub fn server_address(&self) -> Option<String> {
        self.config.get("serveraddress").ok()
    }

    pub fn server_port(&self) -> Option<u16> {
        self.config.get("serverport").ok()
    }

    pub fn redis_host(&self) -> Option<String> {
        self.config.get("redishost").ok()
    }

    pub fn redis_password(&self) -> Option<String> {
        self.config.get("redispassword").ok()
    }

    pub fn redis_port(&self) -> Option<u16> {
        self.config.get("redisport").ok()
    }

    #[allow(dead_code)] // i want to keep this around for now
    pub fn hostname(&self) -> Option<String> {
        self.config.get("hostname").ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_settings() {
        env::remove_var("BOOKSTORE_SERVERADDRESS");
        env::set_var("BOOKSTORE_REDISPASSWORD", "foo");
        env::set_var("BOOKSTORE_REDISHOST", "bar");
        env::set_var("BOOKSTORE_REDISPORT", "6464");
        let settings = Settings::new().unwrap();
        assert!(settings.server_address().is_none());
        assert_eq!("foo", settings.redis_password().unwrap());
        assert_eq!("bar", settings.redis_host().unwrap());
        assert_eq!(6464, settings.redis_port().unwrap());
    }

    #[test]
    #[ignore] // todo - fix
    fn test_server_address_overridden() {
        let addr = "0.0.0.0:80";
        env::set_var("BOOKSTORE_SERVER_ADDRESS", addr);
        assert_eq!("0.0.0.0:80", env::var("BOOKSTORE_SERVERADDRESS").unwrap());
        let settings = Settings::new().unwrap();
        assert_eq!("0.0.0.0:80", settings.server_address().unwrap());
    }

}
