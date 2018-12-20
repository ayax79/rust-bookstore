use config::{Config, Environment};
use errors::BookServiceError;
use std::convert::From;

#[derive(Debug, Clone, Default)]
pub struct Settings {
    pub server_address: Option<String>,
    pub server_port: Option<u32>,
    pub redis_host: Option<String>,
    pub redis_password: Option<String>,
    pub redis_port: Option<u32>,
    pub hostname: Option<String>,
}

impl Settings {
    pub fn new() -> Result<Self, BookServiceError> {
        Config::new()
            .merge(Environment::with_prefix("bookstore"))
            .map_err(BookServiceError::from)
            .map(|config| Settings {
                server_address: config.get("serveraddress").ok(),
                server_port: config.get("serverport").ok(),
                redis_host: config.get("redishost").ok(),
                redis_password: config.get("redispassword").ok(),
                redis_port: config.get("redisport").ok(),
                hostname: config.get("hostname").ok(),
            })
    }

    #[allow(dead_code)]
    pub fn with_service_address(self, service_address: &str) -> Self {
        Settings {
            server_address: Some(service_address.to_string()),
            ..self
        }
    }

    #[allow(dead_code)]
    pub fn with_service_port(self, service_port: u32) -> Self {
        Settings {
            server_port: Some(service_port),
            ..self
        }
    }

    #[allow(dead_code)]
    pub fn with_redis_port(self, redis_port: u32) -> Self {
        Settings {
            redis_port: Some(redis_port),
            ..self
        }
    }

    #[allow(dead_code)]
    pub fn with_redis_host(self, redis_host: &str) -> Self {
        Settings {
            redis_host: Some(redis_host.to_string()),
            ..self
        }
    }

    #[allow(dead_code)]
    pub fn with_redis_password(self, redis_password: &str) -> Self {
        Settings {
            redis_password: Some(redis_password.to_string()),
            ..self
        }
    }

    pub fn redis_url(&self) -> Result<String, BookServiceError> {
        match (&self.redis_host, &self.redis_port, &self.redis_password) {
            (Some(host), Some(port), Some(password)) => {
                Ok(format!("redis://:{}@{}:{}", password, host, port))
            }
            (Some(host), Some(port), None) => Ok(format!("redis://{}:{}", host, port)),
            (None, _, _) => Err(BookServiceError::RedisHostError),
            (_, None, _) => Err(BookServiceError::RedisPortError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_redit_url() {
        let test_settings = Settings::default()
            .with_redis_host("myredishost")
            .with_redis_password("myredispass")
            .with_redis_port(6363);

        let url = "redis://:myredispass@myredishost:6363";
        let result = test_settings.redis_url().unwrap();
        assert_eq!(url.to_string(), result);
    }

    #[test]
    fn test_settings() {
        env::remove_var("BOOKSTORE_SERVERADDRESS");
        env::set_var("BOOKSTORE_REDISPASSWORD", "foo");
        env::set_var("BOOKSTORE_REDISHOST", "bar");
        env::set_var("BOOKSTORE_REDISPORT", "6464");
        let settings = Settings::new().unwrap();
        assert!(settings.server_address.is_none());
        assert_eq!("foo", settings.redis_password.unwrap());
        assert_eq!("bar", settings.redis_host.unwrap());
        assert_eq!(6464, settings.redis_port.unwrap());
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
