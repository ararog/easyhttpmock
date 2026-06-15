use std::{collections::HashMap, fmt::Debug, sync::Arc};

use bytes::Bytes;
use caramelo::expect;
use http::{request::Parts, HeaderMap, Method, StatusCode};

use crate::{matchers::HttpMatcher, server::ServerAdapter, EasyHttpMock, HttpMockResult};

/// State container for mock data
pub struct MockState {
    inner: Arc<Mock>,
}

impl MockState {
    #[inline]
    /// Create a new mock state
    pub fn new(mock: Mock) -> Self {
        Self { inner: Arc::new(mock) }
    }

    /// Get a clone of the internal data Arc
    #[inline]
    pub fn inner(&self) -> Arc<Mock> {
        self.inner.clone()
    }

    /// Set the respond for this request
    pub async fn use_on<S: ServerAdapter>(
        self,
        server: &mut EasyHttpMock<S>,
    ) -> HttpMockResult<Self> {
        server
            .register_mock(&self)
            .await?;

        Ok(self)
    }
}

/// Mock struct
pub struct Mock {
    request: RequestMock,
}

impl Mock {
    #[inline]
    /// Create a new mock
    pub fn of(request: RequestMock) -> MockState {
        MockState::new(Self { request })
    }

    #[inline]
    /// Get the request mock
    pub fn request(&self) -> &RequestMock {
        &self.request
    }

    #[inline]
    /// Match this mock with an actual request
    pub fn match_with(&self, request: Request) {
        // TODO: Implement matching logic
        let mut expect = expect(request);
        for (i, matcher) in self
            .request
            .matchers
            .clone()
            .into_iter()
            .enumerate()
        {
            expect = if i == 0 { expect.to_be(matcher) } else { expect.and(matcher) }
        }
    }
}

#[inline]
/// Add a matcher to this request
pub fn given(matcher: HttpMatcher) -> RequestMock {
    RequestMock { matchers: vec![matcher], respond: None }
}

/// Represents a mock request
pub struct RequestMock {
    matchers: Vec<HttpMatcher>,
    respond: Option<Respond>,
}

impl RequestMock {
    #[inline]
    /// Add a matcher to this request
    pub fn and(mut self, matcher: HttpMatcher) -> Self {
        self.matchers
            .push(matcher);
        self
    }

    #[inline]
    /// Check if this request matches the given request
    pub fn matchers(&self) -> &Vec<HttpMatcher> {
        &self.matchers
    }

    #[inline]
    /// Get the respond for this request
    pub fn respond(&self) -> Option<&Respond> {
        self.respond
            .as_ref()
    }

    #[inline]
    /// Set the response for this request
    pub fn will_return(mut self, respond: Respond) -> Self {
        self.respond = Some(respond);
        self
    }
}

/// Extension trait for StatusCode to create responses
pub trait StatusCodeExt {
    /// Create a response builder with this status code
    fn respond(self) -> RespondBuilder;
}

impl StatusCodeExt for StatusCode {
    /// Create a response builder with this status code
    fn respond(self) -> RespondBuilder {
        RespondBuilder { status_code: self, headers: HashMap::new() }
    }
}

/// Represents a mock HTTP request
#[derive(Clone, Debug, PartialEq)]
pub struct Request {
    path: String,
    method: Method,
    headers: HeaderMap,
    query_params: HashMap<String, String>,
    body: Option<Bytes>,
}

impl Request {
    #[inline]
    /// Create a new request builder
    pub fn from_parts(parts: Parts) -> Request {
        Request {
            path: parts
                .uri
                .path()
                .to_string(),
            method: parts.method,
            headers: parts.headers,
            query_params: parts
                .uri
                .query()
                .map(|q| {
                    q.split('&')
                        .filter_map(|pair| {
                            pair.split_once('=')
                                .map(|(k, v)| (k.to_string(), v.to_string()))
                        })
                        .collect()
                })
                .unwrap_or_default(),
            body: None,
        }
    }

    #[inline]
    /// Get the path
    pub fn path(&self) -> &String {
        &self.path
    }

    #[inline]
    /// Get the method
    pub fn method(&self) -> &Method {
        &self.method
    }

    #[inline]
    /// Get the headers
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    #[inline]
    /// Get the query params
    pub fn query_params(&self) -> &HashMap<String, String> {
        &self.query_params
    }

    #[inline]
    /// Get the body
    pub fn body(&self) -> &Option<Bytes> {
        &self.body
    }
}
/// Builder for what represents a response for a request
pub struct RespondBuilder {
    status_code: StatusCode,
    headers: HashMap<String, String>,
}

impl RespondBuilder {
    #[inline]
    /// Set the status code for this response
    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status_code = status;
        self
    }

    #[inline]
    /// Set a header for this response
    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers
            .insert(key.to_string(), value.to_string());
        self
    }

    #[inline]
    /// Set multiple headers for this response
    pub fn with_headers(mut self, entries: &[(&str, &str)]) -> Self {
        for (key, value) in entries {
            self.headers
                .insert(key.to_string(), value.to_string());
        }
        self
    }

    #[inline]
    /// Create an empty response
    pub fn empty(self) -> Respond {
        self.no_body()
    }

    #[inline]
    /// Create a response with no body
    pub fn no_body(self) -> Respond {
        Respond { status_code: self.status_code, headers: self.headers, body: Bytes::new() }
    }

    #[inline]
    /// Create a response with a body
    pub fn with_body(self, body: &[u8]) -> Respond {
        Respond {
            status_code: self.status_code,
            headers: self.headers,
            body: Bytes::from(body.to_vec()),
        }
    }
}

/// Represents how to respond for a request
#[derive(Clone, Debug, PartialEq)]
pub struct Respond {
    status_code: StatusCode,
    headers: HashMap<String, String>,
    body: Bytes,
}

impl Respond {
    /// Initialize respond builder
    #[inline]
    /// Initialize respond builder
    pub fn builder() -> RespondBuilder {
        RespondBuilder { status_code: StatusCode::OK, headers: HashMap::new() }
    }

    #[inline]
    /// Get the status code
    pub fn status_code(&self) -> StatusCode {
        self.status_code
    }

    #[inline]
    /// Get the headers
    pub fn headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }

    #[inline]
    /// Get the body
    pub fn body(&self) -> Bytes {
        self.body.clone()
    }
}
