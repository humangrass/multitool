use serde::{Deserialize, Serialize};
use std::time::Duration;

/// `DatabaseConfig` represents the configuration for connecting to a database.
///
/// This configuration object can be used to set up a connection pool
/// to PostgreSQL or another `sqlx`-supported database.
///
/// Configuration fields can be loaded from various sources, such as YAML, JSON
/// configuration files, or from environment variables.
///
/// Example configuration in YAML:
///
/// ```yaml
/// host: localhost
/// port: 5432
/// username: user
/// password: password
/// database: test
/// max_open_cons: 10
/// min_idle_cons: 5
/// conn_max_lifetime:
///   secs: 900
///   nanos: 0
/// connection_timeout:
///   secs: 15
///   nanos: 0
/// idle_timeout:
///   secs: 3600
///   nanos: 0
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseConfig {
    /// The database host address (e.g. localhost).
    pub host: String,
    /// The port to connect to the database (e.g. 5432 for PostgreSQL).
    pub port: u16,
    /// The username to connect to the database.
    pub username: String,
    /// The password to connect to the database.
    pub password: String,
    /// The name of the database to connect to.
    pub database: String,
    /// The maximum number of open connections in the pool.
    pub max_open_cons: u32,
    /// The minimum number of idle connections in the pool.
    pub min_idle_cons: u32,
    /// The maximum connection lifetime.
    pub conn_max_lifetime: Duration,
    /// The timeout for getting a connection from the pool.
    pub connection_timeout: Duration,
    /// The idle connection timeout in the pool, after which the connection can be closed.
    pub idle_timeout: Duration,
}
