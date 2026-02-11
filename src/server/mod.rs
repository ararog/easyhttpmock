use std::future::Future;

use vetis::{errors::VetisError, Request, Response};

use crate::errors::EasyHttpMockError;

pub mod adapters;

/// Server adapter trait to allow different http server implementations
pub trait ServerAdapter {
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

    /// Start the server
    ///
    /// # Arguments
    ///
    /// * `handler` - The handler function to handle incoming requests
    ///
    /// # Returns
    ///
    /// * `Result<(), EasyHttpMockError>` - The result of the operation
    ///     
    fn start<H, Fut>(&mut self, handler: H) -> impl Future<Output = Result<(), EasyHttpMockError>>
    where
        H: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response, VetisError>> + Send + Sync + 'static;

    /// Stop the server
    ///
    /// # Returns
    ///
    /// * `Result<(), EasyHttpMockError>` - The result of the operation
    ///     
    fn stop(&mut self) -> impl Future<Output = Result<(), EasyHttpMockError>>;
}

pub trait PortGenerator<S>
where
    S: ServerAdapter,
    S::Config: Clone,
{
    fn random_port() -> u16 {
        rand::random_range(9000..65535)
    }

    fn with_random_port(self) -> Self;
}
