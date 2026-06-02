use std::{error::Error, sync::Arc};

use crate::{errors::EasyHttpMockError, mock::Mock, server::ServerAdapter, EasyHttpMock};

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

#[test]
fn test_server() -> Result<(), Box<dyn Error>> {
    let mock_server = EasyHttpMock::<TestServer>::new(crate::config::EasyHttpMockConfig {
        server_config: TestServerConfig { port: 8080, interface: "127.0.0.1".to_string() },
        base_url: Some("http://127.0.0.1:8080".to_string()),
    })?;

    assert_eq!(
        mock_server
            .config
            .server_config
            .port,
        8080,
        "server port should be 8080"
    );

    Ok(())
}
