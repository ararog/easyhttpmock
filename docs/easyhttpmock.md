---
layout: default
title: EasyHttpMock - The effortless HTTP mock server API
nav_order: 1
---

## EasyHttpMock

**EasyHttpMock** is a powerful yet simple HTTP mock server designed specifically for testing HTTP clients. Built to work with any web server, it provides a clean, intuitive API for creating realistic mock endpoints that simulate real-world API behavior, making your testing workflow faster and more reliable.

## Installation

Add EasyHttpMock to your `Cargo.toml`:

```toml
[dependencies]
easyhttpmock = "0.1.0"
```

## Usage

```rust
use crate::{
    config::EasyHttpMockConfig,
    errors::EasyHttpMockError,
    mock::Mock,
    server::{PortGenerator, ServerAdapter},
    EasyHttpMock,
};
use std::{error::Error, sync::Arc};

#[derive(Debug, Clone)]
pub struct TestServerConfig {
    port: u32,
    interface: String,
}

impl Default for TestServerConfig {
    fn default() -> Self {
        Self { port: 8080, interface: "127.0.0.1".to_string() }
    }
}

impl TestServerConfig {
    pub fn port(&self) -> u32 {
        self.port
    }

    pub fn interface(&self) -> &str {
        &self.interface
    }
}

pub struct TestServer {
    config: TestServerConfig,
    mock: Option<Arc<Mock>>,
}

impl ServerAdapter for TestServer {
    type Config = TestServerConfig;

    fn new(config: Self::Config) -> Result<Self, EasyHttpMockError> {
        Ok(Self { config, mock: None })
    }

    fn hostname(&self) -> String {
        "localhost".to_string()
    }

    fn base_url(&self) -> String {
        format!("http://{}:{}", self.hostname(), self.config.port)
    }

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn register_mock(&mut self, mock: Arc<Mock>) {
        self.mock = Some(mock);
    }

    async fn start(&mut self) -> Result<(), EasyHttpMockError> {
        todo!()
    }

    async fn stop(&mut self) -> Result<(), EasyHttpMockError> {
        todo!()
    }
}

impl PortGenerator<TestServer> for TestServerConfig {
    fn with_random_port(self) -> Self {
        let port = rand::random_range(9000..65535);
        Self { port, ..self }
    }
}
```

## API Reference

For detailed API documentation, see the [docs.rs page](https://docs.rs/easyhttpmock).
