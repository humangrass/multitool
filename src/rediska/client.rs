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
    ///         connection_url: None,
    ///         host: Option::from("127.0.0.1".to_string()),
    ///         port: Option::from(6379),
    ///         username: Option::from("username".to_string()),
    ///         password: Some("top_secret_password".to_string()),
    ///         db: Option::from(0),
    ///         connection_timeout: std::time::Duration::from_secs(60),
    ///         connection_pool_size: 10,
    ///     };
    ///
    ///     let redis_client = Rediska::new(config).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn new(config: RedisConfig) -> Result<Self, anyhow::Error> {
        config.check()?;

        let connection_url = if let Some(url) = &config.connection_url {
            url.clone()
        } else {
            let password_part = if let Some(ref password) = config.password {
                format!(":{}", password)
            } else {
                String::new()
            };
            let auth_part = if let Some(ref username) = config.username {
                format!("{}{}", username, password_part)
            } else {
                password_part
            };
            format!(
                "redis://{}@{}:{}/{}",
                auth_part,
                config.host.as_ref().unwrap(),
                config.port.unwrap(),
                config.db.unwrap_or(0)
            )
        };

        let manager = RedisConnectionManager::new(connection_url)?;
        let pool = Pool::builder()
            .max_size(config.connection_pool_size)
            .connection_timeout(config.connection_timeout)
            .build(manager)
            .await?;

        Ok(Rediska { pool })
    }
}

impl Rediska {
    /// Retrieves a connection from the Redis pool.
    ///
    /// This method provides direct access to a Redis connection, allowing developers
    /// to execute any Redis command available in the `redis` crate. This approach offers
    /// flexibility for performing operations not explicitly implemented in `Rediska`.
    ///
    /// # Examples
    ///
    /// ## Using `del` to delete a key:
    ///
    /// ```no_run
    /// use multitool_hg::rediska::config::RedisConfig;
    /// use multitool_hg::rediska::client::Rediska;
    /// use redis::AsyncCommands;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let config = RedisConfig {
    ///         connection_url: None,
    ///         host: Some("127.0.0.1".to_string()),
    ///         port: Some(6379),
    ///         username: None,
    ///         password: Some("top_secret_password".to_string()),
    ///         db: Some(0),
    ///         connection_timeout: std::time::Duration::from_secs(60),
    ///         connection_pool_size: 10,
    ///     };
    ///
    ///     let redis_client = Rediska::new(config).await?;
    ///     let mut conn = redis_client.conn().await?;
    ///
    ///     let _: () = conn.del("key_to_delete").await?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Using `expire` to set a TTL on a key:
    ///
    /// ```no_run
    /// use multitool_hg::rediska::config::RedisConfig;
    /// use multitool_hg::rediska::client::Rediska;
    /// use redis::AsyncCommands;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let config = RedisConfig {
    ///         connection_url: None,
    ///         host: Some("127.0.0.1".to_string()),
    ///         port: Some(6379),
    ///         username: None,
    ///         password: Some("top_secret_password".to_string()),
    ///         db: Some(0),
    ///         connection_timeout: std::time::Duration::from_secs(60),
    ///         connection_pool_size: 10,
    ///     };
    ///
    ///     let redis_client = Rediska::new(config).await?;
    ///     let mut conn = redis_client.conn().await?;
    ///
    ///     let _: () = conn.expire("key_with_ttl", 120).await?; // Key expires in 120 seconds
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Using `incr` to increment a key’s value:
    ///
    /// ```no_run
    /// use multitool_hg::rediska::config::RedisConfig;
    /// use multitool_hg::rediska::client::Rediska;
    /// use redis::AsyncCommands;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let config = RedisConfig {
    ///         connection_url: None,
    ///         host: Some("127.0.0.1".to_string()),
    ///         port: Some(6379),
    ///         username: None,
    ///         password: Some("top_secret_password".to_string()),
    ///         db: Some(0),
    ///         connection_timeout: std::time::Duration::from_secs(60),
    ///         connection_pool_size: 10,
    ///     };
    ///
    ///     let redis_client = Rediska::new(config).await?;
    ///     let mut conn = redis_client.conn().await?;
    ///
    ///     let new_value: i64 = conn.incr("counter_key", 1).await?;
    ///     println!("New counter value: {}", new_value);
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Returns
    ///
    /// A `Result` with a pooled Redis connection if successful, or an `anyhow::Error` if the connection cannot be obtained.
    pub async fn conn(&self) -> anyhow::Result<bb8::PooledConnection<'_, RedisConnectionManager>> {
        self.pool.get().await.map_err(anyhow::Error::from)
    }

    /// Sets a value in Redis for the given key with an optional TTL.
    ///
    /// # Arguments
    ///
    /// * `key` - The key under which the value should be stored.
    /// * `value` - The value to store in Redis.
    /// * `ttl` - An optional TTL in seconds. If `None`, the key will not expire.
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
    ///         connection_url: None,
    ///         host: Option::from("127.0.0.1".to_string()),
    ///         port: Option::from(6379),
    ///         username: None,
    ///         password: Some("top_secret_password".to_string()),
    ///         db: Option::from(0),
    ///         connection_timeout: std::time::Duration::from_secs(60),
    ///         connection_pool_size: 10,
    ///     };
    ///
    ///     let redis_client = Rediska::new(config).await?;
    ///     redis_client.set("key_with_ttl", "Key will expire in 1 hour", Some(3600)).await?;
    ///     redis_client.set("another_key", "Key will never expire", None).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn set(&self, key: &str, value: &str, ttl: Option<u64>) -> anyhow::Result<()> {
        let mut conn = self.pool.get().await?;
        if let Some(seconds) = ttl {
            let _: () = conn.set_ex(key, value, seconds).await?;
        } else {
            let _: () = conn.set(key, value).await?;
        }
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
    ///     let config = RedisConfig {
    ///         connection_url: None,
    ///         host: Option::from("127.0.0.1".to_string()),
    ///         port: Option::from(6379),
    ///         username: None,
    ///         password: None,
    ///         db: Option::from(0),
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
