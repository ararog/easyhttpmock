use std::future::Future;

use vetis::{errors::VetisError, Request, Response};

use crate::errors::EasyHttpMockError;

pub mod adapters;

pub trait ServerAdapter {
    type Config: Clone;

    fn new(config: Self::Config) -> Result<Self, EasyHttpMockError>
    where
        Self: Sized;

    fn base_url(&self) -> String;

    fn config(&self) -> &Self::Config;

    fn start<H, Fut>(&mut self, handler: H) -> impl Future<Output = Result<(), EasyHttpMockError>>
    where
        H: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response, VetisError>> + Send + Sync + 'static;

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
