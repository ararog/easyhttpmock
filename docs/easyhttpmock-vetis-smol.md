---
layout: default
title: EasyHttpMock Vetis Smol - HTTP mock server using Vetis server with Smol runtime
nav_order: 2
---

# EasyHttpMock Vetis Smol

**EasyHttpMock Vetis Smol** is a powerful yet simple HTTP mock server designed specifically for testing HTTP clients. It provides a clean, intuitive API for creating realistic mock endpoints that simulate real-world API behavior, making your testing workflow faster and more reliable. This implementation uses the Vetis server with the Smol runtime.

## Installation

Add EasyHttpMock Vetis Smol to your `Cargo.toml`:

```toml
[dependencies]
easyhttpmock-vetis-smol = { version = "0.1.0", features = ["http2", "rust-tls"] }
macro_rules_attribute = "0.2.2"
smol = { version = "2.0.2", default-features = false }
smol-macros = { version = "0.1.1", default-features = false }
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

use easyhttpmock_vetis_smol::vetis_adapter::{VetisAdapter, VetisAdapterConfig};
use vetis_smol::Protocol;

use macro_rules_attribute::apply;
use smol_macros::main;

#[apply(main!)]
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
    )
    .use_on(&mut server)
    .await?;

    // TODO: Make a request to the server and assert the response

    server.stop().await?;

    Ok(())
}
```

## API Reference

For detailed API documentation, see the [docs.rs page](https://docs.rs/easyhttpmock-vetis-smol).
