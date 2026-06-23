---
layout: default
title: EasyHttpMock Vetis Smol - HTTP mock server using Vetis server with Smol runtime
nav_order: 2
---

## EasyHttpMock Vetis Smol

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
use easyhttpmock_vetis_smol::{
    EasyHttpMock,
    config::EasyHttpMockConfig,
    matchers::{method, path},
    mock::{AsyncMatcherExt, Mock, StatusCodeExt, given},
    vetis_adapter::VetisAdapter,
};
use http::{Method, StatusCode};
use macro_rules_attribute::apply;
use smol_macros::main;
use std::error::Error;

#[apply(main!)]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = EasyHttpMockConfig::<VetisAdapter>::default();
    let server = EasyHttpMock::new(config);
    let mut server = match server {
        Ok(server) => server,
        Err(err) => {
            panic!("Failed to create mock server: {}", err);
        }
    };

    let mock = Mock::of(
        given(method(Method::GET).and(path("/test"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"teste"),
        ),
    );

    server.register_mock(mock).await?;

    Ok(())
}
```

## API Reference

For detailed API documentation, see the [docs.rs page](https://docs.rs/easyhttpmock-vetis-smol).
