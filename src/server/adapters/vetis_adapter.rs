use std::future::Future;

use vetis::{
    config::{ListenerConfig, SecurityConfig, ServerConfig, VirtualHostConfig},
    errors::VetisError,
    server::{
        path::HandlerPath,
        virtual_host::{handler_fn, VirtualHost},
    },
    Request, Response, Vetis,
};

use crate::{
    config::EasyHttpMockConfig,
    errors::{EasyHttpMockError, ServerError},
    server::{PortGenerator, ServerAdapter},
    EasyHttpMock,
};

pub struct VetisAdapterConfigBuilder {
    interface: String,
    port: u16,
    cert: Option<Vec<u8>>,
    key: Option<Vec<u8>>,
    ca: Option<Vec<u8>>,
}

impl VetisAdapterConfigBuilder {
    pub fn interface(mut self, interface: &str) -> Self {
        self.interface = interface.to_string();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn cert(mut self, cert: Option<Vec<u8>>) -> Self {
        self.cert = cert;
        self
    }

    pub fn key(mut self, key: Option<Vec<u8>>) -> Self {
        self.key = key;
        self
    }

    pub fn ca(mut self, ca: Option<Vec<u8>>) -> Self {
        self.ca = ca;
        self
    }

    pub fn build(self) -> VetisAdapterConfig {
        VetisAdapterConfig {
            interface: self.interface,
            port: self.port,
            cert: self.cert,
            key: self.key,
            ca: self.ca,
        }
    }
}

#[derive(Clone)]
pub struct VetisAdapterConfig {
    interface: String,
    port: u16,
    cert: Option<Vec<u8>>,
    key: Option<Vec<u8>>,
    ca: Option<Vec<u8>>,
}

impl Default for VetisAdapterConfig {
    fn default() -> Self {
        Self { interface: "0.0.0.0".into(), port: 80, cert: None, key: None, ca: None }
    }
}

impl VetisAdapterConfig {
    pub fn builder() -> VetisAdapterConfigBuilder {
        VetisAdapterConfigBuilder {
            interface: "0.0.0.0".into(),
            port: 80,
            cert: None,
            key: None,
            ca: None,
        }
    }

    pub fn interface(&self) -> &str {
        &self.interface
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn cert(&self) -> &Option<Vec<u8>> {
        &self.cert
    }

    pub fn key(&self) -> &Option<Vec<u8>> {
        &self.key
    }

    pub fn ca(&self) -> &Option<Vec<u8>> {
        &self.ca
    }
}

impl From<VetisAdapterConfig> for ServerConfig {
    fn from(config: VetisAdapterConfig) -> Self {
        let listener_config = ListenerConfig::builder()
            .interface(&config.interface)
            .port(config.port)
            .build();
        ServerConfig::builder()
            .add_listener(listener_config)
            .build()
    }
}

pub struct VetisAdapter {
    server: Vetis,
    config: VetisAdapterConfig,
}

impl PortGenerator<VetisAdapter> for VetisAdapterConfigBuilder {
    fn with_random_port(self) -> Self {
        let port = rand::random_range(9000..65535);
        self.port(port)
    }
}

impl Default for EasyHttpMockConfig<VetisAdapter> {
    fn default() -> Self {
        let server_config = VetisAdapterConfig::builder()
            .interface("0.0.0.0")
            .cert(None)
            .key(None)
            .ca(None)
            .port(80)
            .build();
        EasyHttpMockConfig::<VetisAdapter>::builder()
            .server_config(server_config.clone())
            .base_url(format!("http://localhost:{}", 80).into())
            .build()
    }
}

impl Default for EasyHttpMock<VetisAdapter> {
    fn default() -> Self {
        EasyHttpMock::new(EasyHttpMockConfig::default()).unwrap()
    }
}

impl ServerAdapter for VetisAdapter {
    type Config = VetisAdapterConfig;

    fn new(config: Self::Config) -> Result<Self, EasyHttpMockError> {
        let vetis_config = config
            .clone()
            .into();

        let server = Vetis::new(vetis_config);

        Ok(Self { server, config })
    }

    fn base_url(&self) -> String {
        if self
            .config
            .cert
            .is_some()
        {
            format!("https://localhost:{}", self.config.port())
        } else {
            format!("http://localhost:{}", self.config.port())
        }
    }

    fn config(&self) -> &Self::Config {
        &self.config
    }

    async fn start<H, Fut>(&mut self, handler: H) -> Result<(), EasyHttpMockError>
    where
        H: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response, VetisError>> + Send + Sync + 'static,
    {
        let path = HandlerPath::new_host_path("/", handler_fn(handler));

        let host_config = VirtualHostConfig::builder()
            .hostname("localhost")
            .port(self.config.port());

        let host_config = if let Some(((cert, key), ca)) = self
            .config
            .cert
            .as_ref()
            .zip(
                self.config
                    .key
                    .as_ref(),
            )
            .zip(
                self.config
                    .ca
                    .as_ref(),
            ) {
            host_config.security(
                SecurityConfig::builder()
                    .cert_from_bytes(cert.clone())
                    .key_from_bytes(key.clone())
                    .ca_cert_from_bytes(ca.clone())
                    .build(),
            )
        } else {
            host_config
        };

        let host_config = host_config
            .build()
            .map_err(|e| EasyHttpMockError::Server(ServerError::Creation(e.to_string())))?;

        let mut host = VirtualHost::new(host_config);
        host.add_path(path);

        self.server
            .add_virtual_host(host)
            .await;

        self.server
            .start()
            .await
            .map_err(|e| EasyHttpMockError::Server(ServerError::Start(e.to_string())))
    }

    async fn stop(&mut self) -> Result<(), EasyHttpMockError> {
        self.server
            .stop()
            .await
            .map_err(|e| EasyHttpMockError::Server(ServerError::Stop(e.to_string())))
    }
}
