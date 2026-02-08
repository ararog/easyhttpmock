use std::future::Future;

use crate::{config::EasyHttpMockConfig, errors::EasyHttpMockError, server::ServerAdapter};

use bytes::Bytes;
use http::StatusCode;
use http_body_util::{Either, Full};
use vetis::{errors::VetisError, Request, Response};

pub mod config;
pub mod errors;
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

    pub async fn start<H, Fut>(&mut self, handler: H) -> Result<(), EasyHttpMockError>
    where
        H: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response, VetisError>> + Send + Sync + 'static,
    {
        self.server
            .start(handler)
            .await
    }

    pub async fn stop(&mut self) -> Result<(), EasyHttpMockError> {
        self.server
            .stop()
            .await
    }
}

pub fn mock_response(status: StatusCode, body: &str) -> Response {
    Response::builder()
        .status(status)
        .text(body)
}
