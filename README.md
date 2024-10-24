# Multitool

Useful module for creating services on Rust.

<img src="assets/multitool.png" alt="multitool" style="width:400px;"/>

# Features

### 1. Database

The `database` module provides functionality for working with PostgreSQL using asynchronous connection pooling
via `sqlx`. This module includes:

- Database connection settings via `DatabaseConfig`.
- Connection pooling functionality for PostgreSQL.
- `TODO: mysql`

To enable PostgreSQL support, use the `full` or `database` feature. Available by default.

### 2. Logger

The `logger` module provides a logging system based on `tracing-subscriber` and supports different logging
levels (`Info`, `Debug`, `Error`, etc.). You can configure logging levels via `LogLevel` and use them to output
structured logs.

To enable logging, use the `full` or `logger` feature. Available by default.

### 3. Rediska

The `rediska` module provides functionality for working with Redis using asynchronous connection pooling via `bb8`. This module includes:

- Redis connection settings via `RedisConfig`.
- Connection pooling for Redis.
- Convenient methods for setting and getting values in Redis.

The `RedisConfig` allows you to configure parameters like the host, port, username, password, database, connection timeout, and pool size.

To enable Redis support, use the `full` or `rediska` features. Available by default.

# Usage

Documentation is available [here](https://docs.rs/multitool-hg/latest/multitool_hg/).

# Testing

You can run tests for all modules using the `full` feature:

```bash
cargo test --features full
```

Checking the correctness of the build:

```bash
cargo build --features full
```
