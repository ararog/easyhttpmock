---
layout: default
title: EasyHttpMock Vetis Tokio - HTTP mock server using Vetis server with Tokio runtime
nav_order: 3
---

# EasyHttpMock Vetis Tokio

**EasyHttpMock Vetis Tokio** is a powerful yet simple HTTP mock server designed specifically for testing HTTP clients. It provides a clean, intuitive API for creating realistic mock endpoints that simulate real-world API behavior, making your testing workflow faster and more reliable. This implementation uses the Vetis server with the Tokio runtime.

## Installation

Add EasyHttpMock Vetis Tokio to your `Cargo.toml`:

```toml
[dependencies]
easyhttpmock-vetis-tokio = { version = "0.1.0", features = ["http2", "rust-tls"] }
```

## Usage

```rust,no_run
use std::error::Error;

use easyhttpmock::{
    config::EasyHttpMockConfig,
    mock::{MethodExt, Mock, StatusCodeExt},
    server::PortGenerator,
    EasyHttpMock,
};
use http::{Method, StatusCode};

use easyhttpmock_vetis_tokio::vetis_adapter::{VetisAdapter, VetisAdapterConfig};
use vetis_tokio::Protocol;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vetis_adapter_config = VetisAdapterConfig::builder()
        .protocol(Protocol::Http2)
        .with_random_port()
        .build();

    let config = EasyHttpMockConfig::<VetisAdapter>::builder()
        .server_config(vetis_adapter_config)
        .build();

    let server = EasyHttpMock::new(config);
    let mut server = match server {
        Ok(server) => server,
        Err(err) => {
            panic!("Failed to create mock server: {}", err);
        }
    };

    let mock = Mock::of(
        Method::GET
            .has()
            .path("/test")
            .will_return(
                StatusCode::OK
                    .respond()
                    .with_body(b"teste"),
            ),
    );

    server.register_mock(&mock);

    Ok(())
}
```

## API Reference

For detailed API documentation, see the [docs.rs page](https://docs.rs/easyhttpmock-vetis-tokio).
