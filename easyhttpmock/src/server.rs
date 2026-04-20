use std::future::Future;

use crate::{errors::EasyHttpMockError, mock::Mock};

/// Server adapter trait to allow different http server implementations
pub trait ServerAdapter {
    /// The configuration for the server adapter
    type Config: Clone;

    /// Create a new server adapter
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration for the server adapter
    ///
    /// # Returns
    ///
    /// * `Result<Self, EasyHttpMockError>` - The server adapter or an error
    fn new(config: Self::Config) -> Result<Self, EasyHttpMockError>
    where
        Self: Sized;

    /// Get the hostname of the server
    ///
    /// # Returns
    ///
    /// * `String` - The hostname of the server
    fn hostname(&self) -> String;

    /// Get the base URL of the server
    ///
    /// # Returns
    ///
    /// * `String` - The base URL of the server
    ///         
    fn base_url(&self) -> String;

    /// Get the configuration of the server
    ///
    /// # Returns
    ///
    /// * `&Self::Config` - The configuration of the server
    ///     
    fn config(&self) -> &Self::Config;

    /// Set the mocker to handle incoming requests
    ///
    /// # Arguments
    ///
    /// * `mocker` - The mocker to handle incoming requests
    ///
    /// # Returns
    ///
    /// * `Result<(), EasyHttpMockError>` - The result of the operation
    ///     
    fn register_mock(&mut self, mock: Mock);

    /// Start the server
    ///
    /// # Returns
    ///
    /// * `Result<(), EasyHttpMockError>` - The result of the operation
    ///     
    fn start(&mut self) -> impl Future<Output = Result<(), EasyHttpMockError>>;

    /// Stop the server
    ///
    /// # Returns
    ///
    /// * `Result<(), EasyHttpMockError>` - The result of the operation
    ///     
    fn stop(&mut self) -> impl Future<Output = Result<(), EasyHttpMockError>>;
}

/// Port generator trait to allow different port generation strategies
pub trait PortGenerator<S>
where
    S: ServerAdapter,
    S::Config: Clone,
{
    /// Generate a random port
    fn random_port() -> u16 {
        rand::random_range(9000..65535)
    }

    /// Set the server to use a random port
    fn with_random_port(self) -> Self;
}
