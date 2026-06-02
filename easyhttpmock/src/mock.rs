use std::{collections::HashMap, fmt::Debug, sync::Arc};

use bytes::Bytes;
use caramelo::{expect, matchers::eq};
use http::{HeaderMap, Method, StatusCode};

use crate::{server::ServerAdapter, EasyHttpMock, HttpMockResult};

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
    request: Request,
}

impl Mock {
    #[inline]
    /// Create a new mock
    pub fn of(request: Request) -> MockState {
        MockState::new(Self { request })
    }

    #[inline]
    /// Get the request
    pub fn request(&self) -> &Request {
        &self.request
    }

    #[inline]
    /// Match this mock with an actual request
    pub fn match_with(&self, request: Request) {
        // TODO: Implement matching logic
        expect(&self.request).to_be(eq(&request));
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

/// Extension trait for HTTP methods to create requests.
/// Allows creating requests using method names as strings or Method enum values.
///
/// # Examples
/// ``` compile_fail
/// use http::Method;
/// use deboa::request::MethodExt;
///
/// // Using Method enum
/// let request = Method::GET.has();
///
/// // Using string
/// let request = "GET".has();
/// ```
pub trait MethodExt {
    /// Create a request builder with this method
    fn has(self) -> RequestBuilder;
}

impl MethodExt for Method {
    #[inline]
    /// Create a request builder with this method
    fn has(self) -> RequestBuilder {
        match self {
            Method::GET => Request::builder(self),
            Method::POST => Request::builder(self),
            Method::PUT => Request::builder(self),
            Method::DELETE => Request::builder(self),
            Method::PATCH => Request::builder(self),
            Method::HEAD => Request::builder(self),
            Method::OPTIONS => Request::builder(self),
            _ => panic!("Method not supported"),
        }
    }
}

impl MethodExt for &str {
    #[inline]
    /// Create a request builder with this method
    fn has(self) -> RequestBuilder {
        match self {
            "GET" | "get" => Request::builder(Method::GET),
            "POST" | "post" => Request::builder(Method::POST),
            "PUT" | "put" => Request::builder(Method::PUT),
            "DELETE" | "delete" => Request::builder(Method::DELETE),
            "PATCH" | "patch" => Request::builder(Method::PATCH),
            "HEAD" | "head" => Request::builder(Method::HEAD),
            "OPTIONS" | "options" => Request::builder(Method::OPTIONS),
            _ => panic!("Method not supported"),
        }
    }
}

/// Builder for creating mock requests
pub struct RequestBuilder {
    path: String,
    method: Method,
    headers: Option<HeaderMap>,
    query_params: Option<HashMap<String, String>>,
    body: Option<Bytes>,
}

impl RequestBuilder {
    #[inline]
    /// Set the path for this request
    pub fn path(mut self, path: &str) -> Self {
        self.path = path.to_string();
        self
    }

    #[inline]
    /// Set the method for this request
    pub fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    #[inline]
    /// Set the headers for this request
    pub fn headers(mut self, headers: HeaderMap) -> Self {
        self.headers = Some(headers);
        self
    }

    #[inline]
    /// Set the query parameters for this request
    pub fn query_params(mut self, query_params: HashMap<String, String>) -> Self {
        self.query_params = Some(query_params);
        self
    }

    #[inline]
    /// Set the body for this request
    pub fn body(mut self, body: &[u8]) -> Self {
        self.body = Some(Bytes::from(body.to_vec()));
        self
    }

    #[inline]
    /// Build the request
    pub fn build(self) -> Request {
        Request {
            path: self.path,
            method: self.method,
            headers: self.headers,
            query_params: self.query_params,
            body: self.body,
            respond: None,
        }
    }

    #[inline]
    /// Set the response for this request
    pub fn will_return(self, respond: Respond) -> Request {
        Request {
            path: self.path,
            method: self.method,
            headers: self.headers,
            query_params: self.query_params,
            body: self.body,
            respond: Some(respond),
        }
    }
}

/// Represents a mock HTTP request
#[derive(Clone)]
pub struct Request {
    path: String,
    method: Method,
    headers: Option<HeaderMap>,
    query_params: Option<HashMap<String, String>>,
    body: Option<Bytes>,
    respond: Option<Respond>,
}

impl Debug for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Request")
            .field("path", &self.path)
            .field("method", &self.method)
            .field("headers", &self.headers)
            .field("query_params", &self.query_params)
            .field("body", &self.body)
            .finish()
    }
}

impl PartialEq for Request {
    fn eq(&self, other: &Self) -> bool {
        let result = self.path == other.path && self.method == other.method;

        if let Some(headers) = &self.headers {
            if let Some(other_headers) = &other.headers {
                if headers != other_headers {
                    return false;
                }
            }
        }

        if let Some(query_params) = &self.query_params {
            if let Some(other_query_params) = &other.query_params {
                if query_params != other_query_params {
                    return false;
                }
            }
        }

        if let Some(body) = &self.body {
            if let Some(other_body) = &other.body {
                if body != other_body {
                    return false;
                }
            }
        }

        result
    }
}

impl Request {
    #[inline]
    /// Create a new request builder
    pub fn builder(method: Method) -> RequestBuilder {
        RequestBuilder {
            path: String::new(),
            method,
            headers: None,
            query_params: None,
            body: None,
        }
    }

    #[inline]
    /// Get the path
    pub fn path(&self) -> &str {
        &self.path
    }

    #[inline]
    /// Get the method
    pub fn method(&self) -> &Method {
        &self.method
    }

    #[inline]
    /// Get the headers
    pub fn headers(&self) -> &Option<HeaderMap> {
        &self.headers
    }

    #[inline]
    /// Get the query params
    pub fn query_params(&self) -> &Option<HashMap<String, String>> {
        &self.query_params
    }

    #[inline]
    /// Get the body
    pub fn body(&self) -> &Option<Bytes> {
        &self.body
    }

    #[inline]
    /// Get the respond
    pub fn respond(&self) -> &Option<Respond> {
        &self.respond
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
#[derive(Clone)]
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
