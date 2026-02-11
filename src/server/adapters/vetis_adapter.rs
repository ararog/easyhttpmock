use std::future::Future;

use vetis::{
    config::{ListenerConfig, Protocol, SecurityConfig, ServerConfig, VirtualHostConfig},
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

/// Builder for VetisAdapterConfig
pub struct VetisAdapterConfigBuilder {
    hostname: Option<String>,
    interface: String,
    protocol: Protocol,
    port: u16,
    cert: Option<Vec<u8>>,
    key: Option<Vec<u8>>,
    ca: Option<Vec<u8>>,
}

impl VetisAdapterConfigBuilder {
    /// Sets the hostname for the server.
    ///
    /// # Arguments
    /// * `hostname` - The hostname to set.
    ///
    /// # Returns
    /// A new `VetisAdapterConfigBuilder` instance with the hostname set.
    pub fn hostname(mut self, hostname: Option<String>) -> Self {
        self.hostname = hostname;
        self
    }

    /// Sets the interface for the server.
    ///
    /// # Arguments
    /// * `interface` - The interface to set.
    ///
    /// # Returns
    /// A new `VetisAdapterConfigBuilder` instance with the interface set.
    pub fn interface(mut self, interface: &str) -> Self {
        self.interface = interface.to_string();
        self
    }

    /// Sets the protocol for the server.
    ///
    /// # Arguments
    /// * `protocol` - The protocol to set.
    ///
    /// # Returns
    /// A new `VetisAdapterConfigBuilder` instance with the protocol set.
    pub fn protocol(mut self, protocol: Protocol) -> Self {
        self.protocol = protocol;
        self
    }

    /// Sets the port for the server.
    ///
    /// # Arguments
    /// * `port` - The port to set.
    ///
    /// # Returns
    /// A new `VetisAdapterConfigBuilder` instance with the port set.
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Sets the certificate for the server.
    ///
    /// # Arguments
    /// * `cert` - The certificate to set.
    ///
    /// # Returns
    /// A new `VetisAdapterConfigBuilder` instance with the certificate set.    
    pub fn cert(mut self, cert: Option<Vec<u8>>) -> Self {
        self.cert = cert;
        self
    }

    /// Sets the key for the server.
    ///
    /// # Arguments
    /// * `key` - The key to set.
    ///
    /// # Returns
    /// A new `VetisAdapterConfigBuilder` instance with the key set.
    pub fn key(mut self, key: Option<Vec<u8>>) -> Self {
        self.key = key;
        self
    }

    /// Sets the CA certificate for the server.
    ///
    /// # Arguments
    /// * `ca` - The CA certificate to set.
    ///
    /// # Returns
    /// A new `VetisAdapterConfigBuilder` instance with the CA certificate set.
    pub fn ca(mut self, ca: Option<Vec<u8>>) -> Self {
        self.ca = ca;
        self
    }

    /// Builds the VetisAdapterConfig from the builder.
    ///
    /// # Returns
    /// A new `VetisAdapterConfig` instance.
    pub fn build(self) -> VetisAdapterConfig {
        VetisAdapterConfig {
            hostname: self.hostname,
            interface: self.interface,
            protocol: self.protocol,
            port: self.port,
            cert: self.cert,
            key: self.key,
            ca: self.ca,
        }
    }
}

/// Configuration for the Vetis adapter.
#[derive(Clone)]
pub struct VetisAdapterConfig {
    hostname: Option<String>,
    interface: String,
    protocol: Protocol,
    port: u16,
    cert: Option<Vec<u8>>,
    key: Option<Vec<u8>>,
    ca: Option<Vec<u8>>,
}

impl Default for VetisAdapterConfig {
    /// Creates a default configuration for the Vetis adapter.
    ///
    /// This function sets up a basic server configuration with:
    /// - Interface: "0.0.0.0"
    /// - Port: 80
    /// - No TLS certificates (HTTP only)
    ///
    /// # Returns
    /// A default `VetisAdapterConfig` instance.  
    fn default() -> Self {
        Self {
            hostname: None,
            interface: "0.0.0.0".into(),
            protocol: Protocol::Http1,
            port: 80,
            cert: None,
            key: None,
            ca: None,
        }
    }
}

impl VetisAdapterConfig {
    /// Creates a new builder for the Vetis adapter configuration.
    ///
    /// This function sets up a basic server configuration with:
    /// - Interface: "0.0.0.0"
    /// - Port: 80
    /// - No TLS certificates (HTTP only)
    ///
    /// # Returns
    /// A new `VetisAdapterConfigBuilder` instance.  
    pub fn builder() -> VetisAdapterConfigBuilder {
        VetisAdapterConfigBuilder {
            hostname: None,
            interface: "0.0.0.0".into(),
            protocol: Protocol::Http1,
            port: 80,
            cert: None,
            key: None,
            ca: None,
        }
    }

    /// Returns the hostname of the server.
    ///
    /// # Returns
    /// The hostname of the server.
    pub fn hostname(&self) -> &Option<String> {
        &self.hostname
    }

    /// Returns the interface of the server.
    ///
    /// # Returns
    /// The interface of the server.
    pub fn interface(&self) -> &str {
        &self.interface
    }

    /// Returns the port of the server.
    ///
    /// # Returns
    /// The port of the server.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Returns the certificate of the server.
    ///
    /// # Returns
    /// The certificate of the server.
    pub fn cert(&self) -> &Option<Vec<u8>> {
        &self.cert
    }

    /// Returns the key of the server.
    ///
    /// # Returns
    /// The key of the server.    
    pub fn key(&self) -> &Option<Vec<u8>> {
        &self.key
    }

    /// Returns the CA certificate of the server.
    ///
    /// # Returns
    /// The CA certificate of the server.
    pub fn ca(&self) -> &Option<Vec<u8>> {
        &self.ca
    }
}

impl From<VetisAdapterConfig> for ServerConfig {
    fn from(config: VetisAdapterConfig) -> Self {
        let listener_config = ListenerConfig::builder()
            .interface(&config.interface)
            .protocol(config.protocol)
            .port(config.port)
            .build()
            .expect("Failed to build listener config");
        ServerConfig::builder()
            .add_listener(listener_config)
            .build()
            .expect("Failed to build server config")
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
    /// Creates a default configuration for the Vetis adapter.
    ///
    /// This function sets up a basic server configuration with:
    /// - Interface: "0.0.0.0"
    /// - Port: 80
    /// - No TLS certificates (HTTP only)
    ///
    /// # Returns
    /// A default `EasyHttpMockConfig` configured for the Vetis adapter.  
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

    /// Creates a new VetisAdapter instance.
    ///
    /// # Arguments
    /// * `config` - The configuration for the adapter.
    ///
    /// # Returns
    /// A new `VetisAdapter` instance.    
    fn new(config: Self::Config) -> Result<Self, EasyHttpMockError> {
        let vetis_config = config
            .clone()
            .into();

        let server = Vetis::new(vetis_config);

        Ok(Self { server, config })
    }

    /// Returns the hostname of the server.
    ///
    /// # Returns
    /// The hostname of the server.
    fn hostname(&self) -> String {
        self.config
            .hostname()
            .clone()
            .unwrap_or_else(|| "localhost".to_string())
    }

    /// Returns the base URL of the server.
    ///
    /// # Returns
    /// The base URL of the server.    
    fn base_url(&self) -> String {
        let hostname = self.hostname();

        if self
            .config
            .cert
            .is_some()
        {
            format!("https://{}:{}", hostname, self.config.port())
        } else {
            format!("http://{}:{}", hostname, self.config.port())
        }
    }

    /// Returns the configuration of the server.
    ///
    /// # Returns
    /// The configuration of the server.    
    fn config(&self) -> &Self::Config {
        &self.config
    }

    /// Starts the server with the given handler.
    ///
    /// # Arguments
    /// * `handler` - The handler to use for the server.
    ///
    /// # Returns
    /// A result indicating whether the server started successfully.    
    async fn start<H, Fut>(&mut self, handler: H) -> Result<(), EasyHttpMockError>
    where
        H: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response, VetisError>> + Send + Sync + 'static,
    {
        let path = HandlerPath::builder()
            .uri("/")
            .handler(handler_fn(handler))
            .build()
            .unwrap();

        let hostname = self.hostname();

        let host_config = VirtualHostConfig::builder()
            .hostname(&hostname)
            .root_directory("src/tests")
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
                    .build()
                    .map_err(|e| EasyHttpMockError::Server(ServerError::Config(e.to_string())))?,
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

    /// Stops the server.
    ///
    /// # Returns
    /// A result indicating whether the server stopped successfully.
    async fn stop(&mut self) -> Result<(), EasyHttpMockError> {
        self.server
            .stop()
            .await
            .map_err(|e| EasyHttpMockError::Server(ServerError::Stop(e.to_string())))
    }
}
