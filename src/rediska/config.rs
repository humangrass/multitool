use std::time::Duration;
use serde::{Deserialize, Serialize};

/// `RedisConfig` represents the configuration for connecting to a Redis instance.
///
/// This configuration object is used to set up a connection pool to Redis, providing
/// necessary details such as the Redis host, port, authentication credentials, and pool settings.
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
/// Fields like `connection_timeout` and `connection_pool_size` control how the connection pool behaves.
#[derive(Debug, Deserialize, Serialize)]
pub struct RedisConfig {
    /// The Redis server host address (e.g. localhost).
    pub host: String,
    /// The port to connect to the Redis server (e.g. 6379 for Redis).
    pub port: u16,
    /// The optional username to authenticate with Redis (mostly used in Redis 6+ with ACLs).
    pub username: Option<String>,
    /// The optional password to authenticate with Redis.
    pub password: Option<String>,
    /// The database number to connect to (default is 0).
    pub db: u64,
    /// The timeout duration for establishing a connection to the Redis server.
    pub connection_timeout: Duration,
    /// The maximum number of connections allowed in the pool.
    pub connection_pool_size: u32,
}
