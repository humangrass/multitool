use std::time::Duration;
use serde::{Deserialize, Serialize};

/// `RedisConfig` represents the configuration for connecting to a Redis instance.
///
/// This configuration object is used to set up a connection pool to Redis, providing
/// necessary details such as the Redis host, port, authentication credentials, and pool settings.
///
/// In more advanced setups, such as Redis clusters or socket connections, the `connection_url` field
/// can be used to provide a direct connection string. If `connection_url` is provided,
/// fields like `host`, `port`, and `db` become optional.
///
/// Configuration fields can be loaded from various sources, such as YAML, JSON
/// configuration files, or from environment variables.
///
/// Example configuration in YAML:
///
/// ```yaml
/// host: localhost
/// port: 6379
/// username: username
/// password: top_secret_password
/// db: 0
/// connection_timeout:
///   secs: 60
///   nanos: 0
/// connection_pool_size: 10
/// ```
///
/// Another example configuration in YAML:
///
/// ```yaml
/// connection_url: "redis://username:password@localhost:6379/0"
/// connection_timeout:
///   secs: 60
///   nanos: 0
/// connection_pool_size: 10
/// ```
///
/// Fields like `connection_timeout` and `connection_pool_size` control how the connection pool behaves.
#[derive(Debug, Deserialize, Serialize)]
pub struct RedisConfig {
    /// Optional direct connection URL (useful for Redis clusters or socket connections).
    /// If `connection_url` is specified, the `host`, `port`, and `db` fields become optional and are not used.
    pub connection_url: Option<String>,
    /// The Redis server host address (e.g., localhost).
    pub host: Option<String>,
    /// The port to connect to the Redis server (e.g., 6379 for Redis).
    pub port: Option<u16>,
    /// The optional username to authenticate with Redis (mostly used in Redis 6+ with ACLs).
    pub username: Option<String>,
    /// The optional password to authenticate with Redis.
    pub password: Option<String>,
    /// The database number to connect to (default is 0).
    pub db: Option<u64>,
    /// The timeout duration for establishing a connection to the Redis server.
    pub connection_timeout: Duration,
    /// The maximum number of connections allowed in the pool.
    pub connection_pool_size: u32,
}

impl RedisConfig {
    /// Checks the validity of the configuration.
    ///
    /// If `connection_url` is provided, the configuration is considered valid.
    /// Otherwise, `host`, `port`, and `db` must be provided for a valid configuration.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the configuration is valid, or an `anyhow::Error` explaining the missing fields.
    pub fn check(&self) -> anyhow::Result<()> {
        if self.connection_url.is_some() {
            Ok(())
        } else if self.host.is_some() && self.port.is_some() && self.db.is_some() {
            Ok(())
        } else {
            Err(anyhow::Error::msg(
                "Either `connection_url` must be provided or fields `host`, `port`, and `db` must be set for Redis connection."
            ))
        }
    }
}
