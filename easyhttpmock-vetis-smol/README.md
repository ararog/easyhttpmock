# EasyHttpMock Vetis Smol

[![Crates.io downloads](https://img.shields.io/crates/d/easyhttpmock-vetis-smol)](https://crates.io/crates/easyhttpmock-vetis-smol) [![crates.io](https://img.shields.io/crates/v/easyhttpmock-vetis-smol?style=flat-square)](https://crates.io/crates/easyhttpmock-vetis-smol) [![Build Status](https://github.com/ararog/easyhttpmock/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/easyhttpmock/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/easyhttpmock-vetis-smol) [![Documentation](https://docs.rs/easyhttpmock-vetis-smol/badge.svg)](https://docs.rs/easyhttpmock-vetis-smol/latest/easyhttpmock_vetis_smol) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ararog/easyhttpmock/blob/main/LICENSE.md)  [![codecov](https://codecov.io/gh/ararog/easyhttpmock/graph/badge.svg?token=T0HSBAPVSI)](https://codecov.io/gh/ararog/easyhttpmock)

This crate provides the core functionality for creating HTTP mock servers using the Smol runtime.

## Quick Start

Add EasyHttpMock Vetis Smol to your `Cargo.toml`:

```toml
easyhttpmock-vetis-smol = { version = "0.1.0-beta.3", features = ["http2", "rust-tls"] }
macro_rules_attribute = "0.2.2"
smol = { version = "2.0.2", default-features = false }
smol-macros = { version = "0.1.1", default-features = false }
```

## Usage Example

Here's how simple it is to create a web server with VeTiS:

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

## License

Licensed under either of

- Apache License, Version 2.0
  (LICENSE-APACHE or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  (LICENSE-MIT or <https://opensource.org/licenses/MIT>)

at your option.

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
