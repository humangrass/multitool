use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use redis::AsyncCommands;
use crate::rediska::config::RedisConfig;

/// `Rediska` is a Redis client that uses connection pooling to interact with a Redis database.
///
/// This struct is built using `bb8` for connection pooling and provides convenient
/// methods for setting and getting values from Redis. The connection is configured
/// via the `RedisConfig` structure, which defines the Redis host, port, and other settings.
pub struct Rediska {
    pool: Pool<RedisConnectionManager>,
}

impl Rediska {
    /// Creates a new `Rediska` instance with the provided `RedisConfig`.
    ///
    /// This method sets up a connection pool to Redis using the configuration provided.
    ///
    /// # Arguments
    ///
    /// * `config` - A `RedisConfig` struct containing the Redis host, port, password, and other settings.
    ///
    /// # Returns
    ///
    /// A `Result` containing either a new instance of `Rediska` or an `anyhow::Error` if the connection setup fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use multitool_hg::rediska::config::RedisConfig;
    /// use multitool_hg::rediska::client::Rediska;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let config = RedisConfig {
    ///         host: "127.0.0.1".to_string(),
    ///         port: 6379,
    ///         username: Option::from("username".to_string()),
    ///         password: Some("top_secret_password".to_string()),
    ///         db: 0,
    ///         connection_timeout: std::time::Duration::from_secs(60),
    ///         connection_pool_size: 10,
    ///     };
    ///
    ///     let redis_client = Rediska::new(config).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn new(config: RedisConfig) -> Result<Self, anyhow::Error> {
        let connection_url = if let Some(ref username) = config.username {
            format!(
                "redis://{}:{}@{}:{}/{}",
                username, config.password.clone().unwrap_or_default(),
                config.host, config.port, config.db
            )
        } else if let Some(ref password) = config.password {
            format!(
                "redis://:{}@{}:{}/{}",
                password, config.host, config.port, config.db
            )
        } else {
            format!("redis://{}:{}/{}", config.host, config.port, config.db)
        };

        let manager = RedisConnectionManager::new(connection_url)?;
        let pool = Pool::builder()
            .max_size(config.connection_pool_size)
            .build(manager)
            .await?;

        Ok(Rediska { pool })
    }
}

impl Rediska {
    /// Sets a value in Redis for the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key under which the value should be stored.
    /// * `value` - The value to store in Redis.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an `anyhow::Error` if the operation fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use multitool_hg::rediska::config::RedisConfig;
    /// use multitool_hg::rediska::client::Rediska;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let config = RedisConfig {
    ///         host: "127.0.0.1".to_string(),
    ///         port: 6379,
    ///         username: None,
    ///         password: Some("top_secret_password".to_string()),
    ///         db: 0,
    ///         connection_timeout: std::time::Duration::from_secs(60),
    ///         connection_pool_size: 10,
    ///     };
    ///
    ///     let redis_client = Rediska::new(config).await?;
    ///     redis_client.set("my_key", "my_value").await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn set(&self, key: &str, value: &str) -> anyhow::Result<()> {
        let mut conn = self.pool.get().await?;
        let _: () = conn.set(key, value).await?;
        Ok(())
    }

    /// Retrieves a value from Redis for the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key whose value should be retrieved.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Option<String>` with the value if it exists, or `None` if the key does not exist.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use multitool_hg::rediska::config::RedisConfig;
    /// use multitool_hg::rediska::client::Rediska;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     use multitool_hg::rediska::config::RedisConfig;
    /// let config = RedisConfig {
    ///         host: "127.0.0.1".to_string(),
    ///         port: 6379,
    ///         username: None,
    ///         password: None,
    ///         db: 0,
    ///         connection_timeout: std::time::Duration::from_secs(60),
    ///         connection_pool_size: 10,
    ///     };
    ///
    ///     let redis_client = Rediska::new(config).await?;
    ///
    ///     if let Some(value) = redis_client.get("my_key").await? {
    ///         println!("Value: {}", value);
    ///     } else {
    ///         println!("Key not found");
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get(&self, key: &str) -> anyhow::Result<Option<String>> {
        let mut conn = self.pool.get().await?;
        let value: Option<String> = conn.get(key).await?;
        Ok(value)
    }
}
