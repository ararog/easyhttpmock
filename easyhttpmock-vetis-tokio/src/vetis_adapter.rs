use std::sync::{Arc, RwLock};

use vetis_tokio::{
    handler_fn,
    http::Response,
    virtual_host::{path::HandlerPath, VirtualHost},
    Protocol, ServerConfig, Vetis,
};

use easyhttpmock::{
    errors::{EasyHttpMockError, MockError, ServerError},
    mock::{ActualRequest, Mock},
    server::{PortGenerator, ServerAdapter},
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
        let listener_config = vetis_tokio::ListenerConfig::builder()
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

/// Vetis adapter implementation
pub struct VetisAdapter {
    server: Vetis,
    config: VetisAdapterConfig,
    mock: Option<Arc<RwLock<Mock>>>,
}

impl PortGenerator<VetisAdapter> for VetisAdapterConfigBuilder {
    fn with_random_port(self) -> Self {
        let port = rand::random_range(9000..65535);
        self.port(port)
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

        Ok(Self { server, config, mock: None })
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

    /// Sets the mock to handle incoming requests.
    ///
    /// # Arguments
    ///
    /// * `mock` - The mock to handle incoming requests.
    ///
    /// # Returns
    ///
    /// * `Result<(), EasyHttpMockError>` - The result of the operation.
    ///
    fn register_mock(&mut self, mock: Mock) {
        self.mock = Some(Arc::new(RwLock::new(mock)));
    }

    /// Starts the server with the given handler.
    ///
    /// # Arguments
    ///
    /// * `handler` - The handler to use for the server.
    ///
    /// # Returns
    ///
    /// A result indicating whether the server started successfully or a `EasyHttpMockError` if it failed.
    ///
    async fn start(&mut self) -> Result<(), EasyHttpMockError> {
        let mock = match self.mock.as_ref() {
            Some(mocker) => mocker,
            None => return Err(MockError::Notfound.into()),
        };

        let mock_clone = mock.clone();
        let path = HandlerPath::builder()
            .uri("/")
            .handler(handler_fn(move |request| {
                // Since handler function is defined here, we need to clone the mocker
                // to move it into the async block
                let mock = mock_clone.clone();
                async move {
                    let (parts, _body) = request.into_parts();

                    let mut mock_write_guard = mock
                        .write()
                        .unwrap();

                    mock_write_guard.match_with(
                        ActualRequest::builder()
                            .path(parts.uri.path())
                            .method(parts.method)
                            .headers(parts.headers)
                            .build(),
                    );

                    mock_write_guard.report_call();

                    drop(mock_write_guard);

                    let mock_read_guard = mock.read().unwrap();
                    let respond = mock_read_guard
                        .request()
                        .respond();

                    Ok(Response::builder()
                        .status(respond.status_code())
                        .bytes(&respond.body()))
                }
            }))
            .build();

        let hostname = self.hostname();

        let host_config = vetis_tokio::VirtualHostConfig::builder()
            .hostname(&hostname)
            .root_directory(".")
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
                vetis_tokio::SecurityConfig::builder()
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
        if let Err(e) = path {
            return Err(EasyHttpMockError::Server(ServerError::Creation(e.to_string())));
        }

        host.add_path(path.unwrap());

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
