---
layout: default
title: EasyHttpMock - The effortless HTTP mock server
nav_order: 1
description: "🧪 The effortless HTTP mock server for seamless API testing"
permalink: /
---
<div align="center">
<h1><b>EasyHttpMock</b></h1>
</div>

[![Crates.io downloads](https://img.shields.io/crates/d/easyhttpmock)](https://crates.io/crates/easyhttpmock) [![crates.io](https://img.shields.io/crates/v/easyhttpmock?style=flat-square)](https://crates.io/crates/easyhttpmock) [![Build Status](https://github.com/ararog/easyhttpmock/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/easyhttpmock/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/easyhttpmock) [![Documentation](https://docs.rs/easyhttpmock/badge.svg)](https://docs.rs/easyhttpmock/latest/easyhttpmock) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ararog/easyhttpmock/blob/main/LICENSE.md)  [![codecov](https://codecov.io/gh/ararog/easyhttpmock/graph/badge.svg?token=T0HSBAPVSI)](https://codecov.io/gh/ararog/easyhttpmock)

**EasyHttpMock** is a powerful yet simple HTTP mock server designed specifically for testing HTTP clients. Built on top of [VeTiS](https://github.com/ararog/vetis), it provides a clean, intuitive API for creating realistic mock endpoints that simulate real-world API behavior, making your testing workflow faster and more reliable.

## Features

- **Testing-Focused**: Purpose-built for HTTP client testing scenarios
- **Lightning Fast**: Powered by VeTiS for optimal performance
- **Flexible Runtime**: Choose between Tokio or Smol async runtimes
- **Full Protocol Support**: HTTP/1, HTTP/2, and HTTP/3 compatibility
- **Secure Testing**: Built-in TLS support for HTTPS endpoint testing
- **Minimal Dependencies**: Lightweight footprint for your test suite

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
easyhttpmock = { version = "0.0.9" }
```

Basic usage:

```rust
use http::StatusCode;

use easyhttpmock::{
    EasyHttpMock,
    config::EasyHttpMockConfig,
    server::{
        PortGenerator,
        adapters::vetis_adapter::{VetisAdapter, VetisAdapterConfig},
    },
};

use deboa::{
    Client,
    cert::{Certificate, ContentEncoding},
    request::DeboaRequest,
};

use vetis::Response;

pub const CA_CERT: &[u8] = include_bytes!("../certs/ca.der");
pub const CA_CERT_PEM: &[u8] = include_bytes!("../certs/ca.crt");

pub const SERVER_CERT: &[u8] = include_bytes!("../certs/server.der");
pub const SERVER_KEY: &[u8] = include_bytes!("../certs/server.key.der");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vetis_adapter_config = VetisAdapterConfig::builder()
        .interface("0.0.0.0")
        .with_random_port()
        .cert(Some(SERVER_CERT.to_vec()))
        .key(Some(SERVER_KEY.to_vec()))
        .ca(Some(CA_CERT.to_vec()))
        .build();

    let config = EasyHttpMockConfig::<VetisAdapter>::builder()
        .server_config(vetis_adapter_config)
        .build();

    let mut server = EasyHttpMock::new(config)?;
    let result = server
        .start(|_| async move {
            Ok(Response::new(Full::new(Bytes::from("Hello World"))))
        })
        .await;

    result.unwrap_or_else(|err| {
        panic!("Failed to start mock server: {}", err);
    });

    let client = Client::builder()
        .certificate(deboa::cert::Certificate::from_slice(CA_CERT, ContentEncoding::DER))
        .build();

    let request = DeboaRequest::get(server.url("/anything"))?.build()?;

    let response = client
        .execute(request)
        .await?;

    if response.status() == StatusCode::OK {
        println!("Request executed successfully");
    }

    server
        .stop()
        .await?;
    
    Ok(())
}    
```

## Examples

Check out the [examples](./examples.md) for complete examples of how to use EasyHttpMock in your projects.

## Create project from template

You can create a new project from the template using `cargo generate`:

`cargo generate ararog/easyhttpmock-templates`

## Documentation

- [API Reference](https://docs.rs/easyhttpmock)
- [Migration Guide](./MIGRATION_GUIDE.md)
- [Contributing Guide](./CONTRIBUTING.md)

## License

This project is licensed under the [MIT License](./LICENSE.md).

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
