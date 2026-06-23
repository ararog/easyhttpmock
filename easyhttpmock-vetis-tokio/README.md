# EasyHttpMock Vetis Tokio

[![Crates.io downloads](https://img.shields.io/crates/d/easyhttpmock-vetis-tokio)](https://crates.io/crates/easyhttpmock-vetis-tokio) [![crates.io](https://img.shields.io/crates/v/easyhttpmock-vetis-tokio?style=flat-square)](https://crates.io/crates/easyhttpmock-vetis-tokio) [![Build Status](https://github.com/ararog/easyhttpmock/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/easyhttpmock/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/easyhttpmock-vetis-tokio) [![Documentation](https://docs.rs/easyhttpmock-vetis-tokio/badge.svg)](https://docs.rs/easyhttpmock-vetis-tokio/latest/easyhttpmock_vetis_tokio) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ararog/easyhttpmock/blob/main/LICENSE.md)  [![codecov](https://codecov.io/gh/ararog/easyhttpmock/graph/badge.svg?token=T0HSBAPVSI)](https://codecov.io/gh/ararog/easyhttpmock)

This crate provides the core functionality for creating HTTP mock servers using the Tokio runtime.

## Quick Start

Add EasyHttpMock Vetis Tokio to your `Cargo.toml`:

```toml
easyhttpmock-vetis-tokio = { version = "0.1.0-beta.3", features = ["http1", "rust-tls"] }
```

## Usage Example

Here's how simple it is to create a web server with VeTiS:

```rust,no_run
use easyhttpmock_vetis_tokio::{
    EasyHttpMock,
    config::EasyHttpMockConfig,
    matchers::{method, path},
    mock::{AsyncMatcherExt, Mock, StatusCodeExt, given},
    vetis_adapter::VetisAdapter,
};
use http::{Method, StatusCode};
use std::error::Error;

#[tokio::main]
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

## License

Licensed under either of

- Apache License, Version 2.0
  (LICENSE-APACHE or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  (LICENSE-MIT or <https://opensource.org/licenses/MIT>)

at your option.

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
