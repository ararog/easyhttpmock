#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
use std::ops::{Deref, DerefMut};

use crate::{
    config::EasyHttpMockConfig, errors::EasyHttpMockError, mock::MockState, server::ServerAdapter,
};

/// Configuration module
pub mod config;
/// Error module
pub mod errors;
/// Matchers module
pub mod matchers;
/// Mock module
pub mod mock;
/// Server module
pub mod server;

#[cfg(test)]
mod tests;

/// Result type for HTTP mock operations
///
/// # Examples
///
/// ```rust,ignore
/// let result: HttpMockResult<()> = Ok(());
/// ```
pub type HttpMockResult<T> = Result<T, EasyHttpMockError>;

/// Create a mock using a specific server implementation
///
/// # Examples
///
/// ```rust,ignore
/// let mut server = EasyHttpMock::new(EasyHttpMockConfig::builder().build());
/// ```
pub struct EasyHttpMock<S>
where
    S: ServerAdapter,
{
    /// Configuration for the mock server
    config: EasyHttpMockConfig<S>,
    /// The actual server implementation
    server: S,
}

impl<S: ServerAdapter> Deref for EasyHttpMock<S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.server
    }
}

impl<S: ServerAdapter> DerefMut for EasyHttpMock<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.server
    }
}

impl<S: ServerAdapter> EasyHttpMock<S> {
    /// Creates a new mock with the given configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration for the mock server
    ///
    /// # Returns
    ///
    /// * `Result<EasyHttpMock<S>, EasyHttpMockError>` - A result indicating whether the mock was created successfully
    ///
    pub fn new(config: EasyHttpMockConfig<S>) -> Result<EasyHttpMock<S>, EasyHttpMockError> {
        let server = S::new(
            config
                .server_config
                .clone(),
        )?;

        Ok(EasyHttpMock { config, server })
    }

    /// Returns the full URL for a given path
    ///
    /// # Arguments
    ///
    /// * `path` - The path to append to the base URL
    ///
    /// # Returns
    ///
    /// * `String` - The full URL for the given path
    pub fn url(&self, path: &str) -> String {
        if let Some(base_url) = &self.config.base_url {
            format!("{}{}", base_url, path)
        } else {
            format!(
                "{}{}",
                self.server
                    .base_url(),
                path
            )
        }
    }

    /// Returns the base URL for the mock server
    ///
    /// # Returns
    ///
    /// * `String` - The base URL for the mock server
    ///
    pub fn base_url(&self) -> String {
        self.server
            .base_url()
    }

    /// Starts the mock server with the given mocker function
    ///
    /// # Arguments
    ///
    /// * `mocker` - A function that returns a `Mock` or an error
    ///
    /// # Returns
    ///
    /// * `Result<(), EasyHttpMockError>` - A result indicating whether the mock server started successfully
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let mut server = EasyHttpMock::new(EasyHttpMockConfig::builder().build());
    /// let mock = Mock::of(
    ///     Method::GET
    ///         .has()
    ///         .path("/test")
    ///         .will_return(
    ///             StatusCode::OK
    ///                 .respond()
    ///                 .with_body(b"teste"),
    ///         ),
    /// );
    ///
    /// server.register_mock(mock);
    /// ```
    pub async fn register_mock(&mut self, mock: MockState) -> HttpMockResult<()> {
        self.server
            .register_mock(mock.inner());

        self.start().await
    }

    /// Stop server
    ///
    /// # Returns
    ///
    /// * `Result<(), EasyHttpMockError>` - A result indicating whether the server stopped successfully
    pub async fn stop(&mut self) -> HttpMockResult<()> {
        self.server
            .stop()
            .await
    }
}
