#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
use crate::{
    config::EasyHttpMockConfig, errors::EasyHttpMockError, mock::MockState, server::ServerAdapter,
};
use caramelo::{MatchType, Matcher, TypedMatcher};
use std::ops::{Deref, DerefMut};
#[cfg(feature = "single-threaded")]
use std::rc::Rc;
#[cfg(feature = "multi-threaded")]
use std::sync::Arc;

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
pub type HttpMockResult<T> = Result<T, EasyHttpMockError>;

#[cfg(feature = "multi-threaded")]
/// Multi threaded typed matcher
pub struct SharedTypedMatcher<T>(Arc<dyn TypedMatcher<T> + Send + Sync>);

#[cfg(feature = "multi-threaded")]
impl<T> Clone for SharedTypedMatcher<T> {
    fn clone(&self) -> Self {
        SharedTypedMatcher(self.0.clone())
    }
}

#[cfg(feature = "multi-threaded")]
impl<T> SharedTypedMatcher<T> {
    /// Creates a shared typed matcher from any matcher implementation.
    pub fn new<M>(matcher: M) -> Self
    where
        M: TypedMatcher<T> + Send + Sync + 'static,
    {
        Self(Arc::new(matcher))
    }

    /// Checks whether the wrapped matcher matches a value.
    pub fn matches(&self, value: &T) -> bool {
        self.0.matches(value)
    }

    /// Returns a human-readable description of the wrapped matcher.
    pub fn description(&self) -> String {
        self.0.description()
    }

    /// Returns the match type for the wrapped matcher.
    pub fn matcher_type(&self) -> MatchType {
        self.0.matcher_type()
    }
}

#[cfg(feature = "multi-threaded")]
impl<T> Matcher<T> for SharedTypedMatcher<T> {
    fn matches(&self, value: &T) -> bool {
        self.0.matches(value)
    }

    fn description(&self) -> String {
        self.0.description()
    }
}

#[cfg(feature = "multi-threaded")]
impl<T> TypedMatcher<T> for SharedTypedMatcher<T> {
    fn matcher_type(&self) -> MatchType {
        self.0.matcher_type()
    }
}

#[cfg(feature = "multi-threaded")]
impl<T> AsRef<dyn TypedMatcher<T> + Send + Sync + 'static> for SharedTypedMatcher<T> {
    fn as_ref(&self) -> &(dyn TypedMatcher<T> + Send + Sync + 'static) {
        self.0.as_ref()
    }
}

#[cfg(feature = "single-threaded")]
/// Multi threaded typed matcher
pub struct SharedTypedMatcher<T>(Rc<dyn TypedMatcher<T>>);

#[cfg(feature = "single-threaded")]
impl<T> Clone for SharedTypedMatcher<T> {
    fn clone(&self) -> Self {
        SharedTypedMatcher(self.0.clone())
    }
}

#[cfg(feature = "single-threaded")]
impl<T> SharedTypedMatcher<T> {
    /// Creates a shared typed matcher from any matcher implementation.
    pub fn new<M>(matcher: M) -> Self
    where
        M: TypedMatcher<T> + 'static,
    {
        Self(Rc::new(matcher))
    }

    /// Checks whether the wrapped matcher matches a value.
    pub fn matches(&self, value: &T) -> bool {
        self.0.matches(value)
    }

    /// Returns a human-readable description of the wrapped matcher.
    pub fn description(&self) -> String {
        self.0.description()
    }

    /// Returns the match type for the wrapped matcher.
    pub fn matcher_type(&self) -> MatchType {
        self.0.matcher_type()
    }
}

#[cfg(feature = "single-threaded")]
impl<T> Matcher<T> for SharedTypedMatcher<T> {
    fn matches(&self, value: &T) -> bool {
        self.0.matches(value)
    }

    fn description(&self) -> String {
        self.0.description()
    }
}

#[cfg(feature = "single-threaded")]
impl<T> TypedMatcher<T> for SharedTypedMatcher<T> {
    fn matcher_type(&self) -> MatchType {
        self.0.matcher_type()
    }
}

#[cfg(feature = "single-threaded")]
impl<T> AsRef<dyn TypedMatcher<T> + 'static> for SharedTypedMatcher<T> {
    fn as_ref(&self) -> &(dyn TypedMatcher<T> + 'static) {
        self.0.as_ref()
    }
}

#[cfg(feature = "multi-threaded")]
/// Trait bounds alias for async typed matchers
pub trait AsyncShared: Send + Sync {}
#[cfg(feature = "multi-threaded")]
impl<T> AsyncShared for T where T: Send + Sync {}

#[cfg(feature = "single-threaded")]
/// Trait bounds alias for async typed matchers
pub trait AsyncShared {}
#[cfg(feature = "single-threaded")]
impl<T> AsyncShared for T {}

#[cfg(feature = "multi-threaded")]
/// Trait bounds alias for async typed matchers
pub trait AsyncTypedMatcher<T>: TypedMatcher<T> + Send + Sync + 'static {}
#[cfg(feature = "multi-threaded")]
impl<T, M> AsyncTypedMatcher<M> for T where T: TypedMatcher<M> + Send + Sync + 'static {}

#[cfg(feature = "single-threaded")]
/// Trait bounds alias for async typed matchers
pub trait AsyncTypedMatcher<T>: TypedMatcher<T> + 'static {}
#[cfg(feature = "single-threaded")]
impl<T, M> AsyncTypedMatcher<M> for T where T: TypedMatcher<M> + 'static {}

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
