use std::{future::Future, pin::Pin};

use crate::{
    config::EasyHttpMockConfig,
    errors::EasyHttpMockError,
    expect::{Then, When},
    server::ServerAdapter,
};

pub mod config;
pub mod errors;
pub mod expect;
pub mod server;

mod tests;

pub struct EasyHttpMock<S>
where
    S: ServerAdapter,
{
    config: EasyHttpMockConfig<S>,
    server: S,
}

impl<S: ServerAdapter> EasyHttpMock<S> {
    pub fn new(config: EasyHttpMockConfig<S>) -> Result<EasyHttpMock<S>, EasyHttpMockError> {
        let server = S::new(
            config
                .server_config
                .clone(),
        )?;

        Ok(EasyHttpMock { config, server })
    }

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

    pub fn base_url(&self) -> String {
        self.server
            .base_url()
    }

    pub async fn mock<F, Fut>(&mut self, mocker: F) -> Result<(), EasyHttpMockError>
    where
        F: Fn(When) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Then, EasyHttpMockError>> + Send + Sync + 'static,
    {
        self.server
            .mocker(mocker);

        self.server
            .start()
            .await
    }

    pub async fn assert(&mut self) -> Result<(), EasyHttpMockError> {
        self.server
            .stop()
            .await
    }
}

/// Type alias for boxed handler closures.
///
/// This represents an async function that takes a `Request` and returns
/// a `Response` or an error. Handlers are the core of request processing
/// in VeTiS virtual hosts.
///
/// # Examples
///
/// ```rust,ignore
/// use easyhttpmock::{errors::EasyHttpMockError, expect::{When, Then}, BoxedHandlerClosure};
///
/// let handler: BoxedHandlerClosure = Box::new(|when: When| {
///     Box::pin(async move {
///         // Process request...
///         Ok(Then::builder()
///             .status(http::StatusCode::OK)
///             .body(http_body_util::Full::new(bytes::Bytes::from("OK"))))
///     })
/// });
/// ```
pub type BoxedHandlerClosure = Box<
    dyn Fn(When) -> Pin<Box<dyn Future<Output = Result<Then, EasyHttpMockError>> + Send + Sync>>
        + Send
        + Sync,
>;

/// Creates a handler closure from a function.
///
/// This utility function converts any compatible async function into a
/// `BoxedHandlerClosure` that can be used with virtual hosts.
///
/// # Arguments
///
/// * `f` - An async function that takes a `Then` and returns a `Result<Then, EasyHttpMockError>`
///
/// # Examples
///
/// ```rust,ignore
///
/// ```
pub fn mock_fn<F, Fut>(f: F) -> BoxedHandlerClosure
where
    F: Fn(When) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<Then, EasyHttpMockError>> + Send + Sync + 'static,
{
    Box::new(move |req| Box::pin(f(req)))
}
