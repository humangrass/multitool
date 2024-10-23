# Multitool

Useful module for creating services on Rust.

![multitool](assets/multitool.png)

# Features

### 1. Database

The `database` module provides functionality for working with PostgreSQL using asynchronous connection pooling
via `sqlx`. This module includes:

- Database connection settings via `DatabaseConfig`.
- Connection pooling functionality for PostgreSQL.
- `TODO: mysql`

To enable PostgreSQL support, use the `database` feature.

### 2. Logger

The `logger` module provides a logging system based on `tracing-subscriber` and supports different logging
levels (`Info`, `Debug`, `Error`, etc.). You can configure logging levels via `LogLevel` and use them to output
structured logs.

To enable logging, use the `logger` feature.

# Testing

You can run tests for all modules using the `full` feature:

```bash
cargo test --features full
```

Checking the correctness of the build:

```bash
cargo build --features full
```
