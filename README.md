# EasyHttpMock

[![Crates.io downloads](https://img.shields.io/crates/d/easyhttpmock)](https://crates.io/crates/easyhttpmock) [![crates.io](https://img.shields.io/crates/v/easyhttpmock?style=flat-square)](https://crates.io/crates/easyhttpmock) [![Build Status](https://github.com/ararog/easyhttpmock/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/easyhttpmock/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/easyhttpmock) [![Documentation](https://docs.rs/easyhttpmock/badge.svg)](https://docs.rs/easyhttpmock/latest/easyhttpmock) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ararog/easyhttpmock/blob/main/LICENSE.md)  [![codecov](https://codecov.io/gh/ararog/easyhttpmock/graph/badge.svg?token=T0HSBAPVSI)](https://codecov.io/gh/ararog/easyhttpmock)


**The effortless HTTP mock server for seamless API testing**

**EasyHttpMock** is a powerful yet simple HTTP mock server designed specifically for testing HTTP clients. Built on top of [VeTiS](https://github.com/ararog/vetis), it provides a clean, intuitive API for creating realistic mock endpoints that simulate real-world API behavior, making your testing workflow faster and more reliable.

## Why EasyHttpMock?

- **Testing-Focused**: Purpose-built for HTTP client testing scenarios
- **Lightning Fast**: Powered by VeTiS for optimal performance
- **Flexible Runtime**: Choose between Tokio or Smol async runtimes
- **Full Protocol Support**: HTTP/1, HTTP/2, and HTTP/3 compatibility
- **Secure Testing**: Built-in TLS support for HTTPS endpoint testing
- **Minimal Dependencies**: Lightweight footprint for your test suite

## Quick Start

Add EasyHttpMock to your `Cargo.toml`:

```rust
easyhttpmock = { version = "0.1.1", features = ["tokio-rt", "http2", "tokio-rust-tls"] }
```

## Usage Example

Here's how simple it is to create a mock HTTP server for testing:

```rust
use bytes::Bytes;
use http::StatusCode;
use http_body_util::Full;
use hyper::Response;

use easyhttpmock::{
    config::EasyHttpMockConfig,
    server::{adapters::vetis_adapter::VetisServerAdapter, PortGenerator},
    EasyHttpMock,
};

use deboa::{cert::ContentEncoding, request::DeboaRequest, Client};

use vetis::{
    server::{
        config::{SecurityConfig, ServerConfig},
    },
};

pub const CA_CERT: &[u8] = include_bytes!("../certs/ca.der");
pub const CA_CERT_PEM: &[u8] = include_bytes!("../certs/ca.crt");

pub const SERVER_CERT: &[u8] = include_bytes!("../certs/server.der");
pub const SERVER_KEY: &[u8] = include_bytes!("../certs/server.key.der");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tls_config = SecurityConfig::builder()
        .cert(SERVER_CERT.to_vec())
        .key(SERVER_KEY.to_vec())
        .build();

    let vetis_config = ServerConfig::builder()
        .security(tls_config)
        .with_random_port()
        .build();

    let config = EasyHttpMockConfig::<VetisServerAdapter>::builder()
        .server_config(vetis_config)
        .build();

    let mut server = EasyHttpMock::new(config);
    #[allow(unused_must_use)]
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

## Perfect For

- **Unit Testing**: Mock external APIs in your test suite
- **Integration Testing**: Test HTTP client behavior without real services
- **Load Testing**: Simulate API responses under various conditions
- **Debugging**: Reproduce API issues in a controlled environment
- **Documentation**: Create interactive API examples

## Supported Runtimes

- [tokio](https://github.com/tokio-rs/tokio) - High-performance async runtime
- [smol](https://github.com/smol-rs/smol) - Lightweight async runtime

## Crate Features

- **tokio-rt** (default) - Tokio runtime support
- **smol-rt** - Smol runtime support  
- **http1** - HTTP/1 protocol support
- **http2** (default) - HTTP/2 protocol support
- **http3** - HTTP/3 protocol support
- **tokio-rust-tls** (default) - TLS support for Tokio
- **smol-rust-tls** - TLS support for Smol

## License

MIT

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>