# Resend API SDK for Rust

This SDK provides a convenient way to interact with the [Resend API](https://resend.com/) using the Rust programming language. It leverages asynchronous programming to efficiently manage API requests.

## Features

- Asynchronous requests using `tokio` and `reqwest`
- JSON serialization and deserialization with `serde` and `serde_json`
- Error handling with `anyhow`
- Environment variable management using `dotenv`

## Installation

Add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json", "blocking"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
dotenv = "0.15"
```

