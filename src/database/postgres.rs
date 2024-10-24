use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::PgPool;

use crate::database::config::DatabaseConfig;

/// Creates a new PostgreSQL connection pool using the provided `DatabaseConfig`.
///
/// This function establishes a connection pool with the PostgreSQL database
/// based on the parameters in the `DatabaseConfig` struct. It allows you to configure
/// the number of connections, timeouts, and connection lifetime.
///
/// ### Parameters
/// - `config`: A `DatabaseConfig` struct containing the necessary details for connecting
///   to the PostgreSQL database, such as host, port, username, password, etc.
///
/// ### Returns
/// A `Result` containing either a `PgPool` on success or an `anyhow::Error` on failure.
///
/// ### Errors
/// This function returns an error if the connection to the database cannot be established,
/// or if the connection pool options are invalid.
///
/// ### Example
///
/// ```no_run
/// use multitool_hg::database::config::DatabaseConfig;
/// use multitool_hg::database::postgres::new_postgres_pool;
/// use std::time::Duration;
///
/// #[tokio::main]
/// async fn main() -> Result<(), anyhow::Error> {
///     let config = DatabaseConfig {
///         host: "127.0.0.1".to_string(),
///         port: 5432,
///         username: "user".to_string(),
///         password: "password".to_string(),
///         database: "test".to_string(),
///         max_open_cons: 10,
///         min_idle_cons: 5,
///         conn_max_lifetime: Duration::from_secs(900),
///         connection_timeout: Duration::from_secs(15),
///         idle_timeout: Duration::from_secs(3600),
///     };
///
///     let pool = new_postgres_pool(config).await?;
///
///     Ok(())
/// }
/// ```
pub async fn new_postgres_pool(config: DatabaseConfig) -> Result<PgPool, anyhow::Error> {
    let connect_options = PgConnectOptions::new()
        .username(&config.username)
        .password(&config.password)
        .host(&config.host)
        .port(config.port)
        .database(&config.database);

    let pool = PgPoolOptions::new()
        .max_connections(config.max_open_cons)
        .min_connections(config.min_idle_cons)
        .acquire_timeout(config.connection_timeout)
        .max_lifetime(config.conn_max_lifetime)
        .idle_timeout(config.idle_timeout)
        .connect_with(connect_options)
        .await?;
    Ok(pool)
}
