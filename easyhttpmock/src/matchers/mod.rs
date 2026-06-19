mod body;
mod header;
mod method;
mod path;
mod query;

pub use body::*;
use caramelo::{MatchType::ToHave, Matcher, TypedMatcher};
pub use header::*;
pub use method::*;
pub use path::*;
pub use query::*;

use crate::mock::Request;

#[derive(Clone)]
/// Enum representing different types of HTTP matchers
pub enum HttpMatcher {
    /// Matches the request path
    Path(Path),
    /// Matches the HTTP method
    Method(Method),
    /// Matches a specific header
    Header(Header),
    /// Matches a specific header value
    HeaderValue(HeaderValue),
    /// Matches the JWT token
    Jwt(Jwt),
    /// Matches the Basic auth token
    BasicAuth(BasicAuth),
    /// Matches a specific query parameter
    QueryParam(QueryParam),
    /// Matches a specific query value
    QueryValue(QueryValue),
    /// Matches the request body
    Body(Body),
    #[cfg(feature = "json")]
    /// Matches the request body as JSON
    ExactJson(BodyWithExactJson),
    #[cfg(feature = "json")]
    /// Matches the request body as JSON
    PartialJson(BodyWithPartialJson),
    #[cfg(feature = "xml")]
    /// Matches the request body as XML
    ExactXml(BodyWithExactXml),
    #[cfg(feature = "json")]
    /// Matches the request body as XML
    PartialXml(BodyWithPartialXml),
}

impl Matcher<Request> for HttpMatcher {
    fn matches(&self, value: &Request) -> bool {
        match self {
            HttpMatcher::Path(path) => path.matches(value),
            HttpMatcher::Method(method) => method.matches(value),
            HttpMatcher::Header(header) => header.matches(value),
            HttpMatcher::HeaderValue(header_value) => header_value.matches(value),
            HttpMatcher::Jwt(jwt) => jwt.matches(value),
            HttpMatcher::BasicAuth(basic_auth) => basic_auth.matches(value),
            HttpMatcher::QueryParam(query) => query.matches(value),
            HttpMatcher::QueryValue(query) => query.matches(value),
            HttpMatcher::Body(body) => body.matches(value),
            #[cfg(feature = "json")]
            HttpMatcher::ExactJson(json) => json.matches(value),
            #[cfg(feature = "json")]
            HttpMatcher::PartialJson(json) => json.matches(value),
            #[cfg(feature = "xml")]
            HttpMatcher::ExactXml(xml) => xml.matches(value),
            #[cfg(feature = "xml")]
            HttpMatcher::PartialXml(xml) => xml.matches(value),
        }
    }

    fn description(&self) -> String {
        match self {
            HttpMatcher::Path(path) => path.description(),
            HttpMatcher::Method(method) => method.description(),
            HttpMatcher::Header(header) => header.description(),
            HttpMatcher::HeaderValue(header_value) => header_value.description(),
            HttpMatcher::Jwt(jwt) => jwt.description(),
            HttpMatcher::BasicAuth(basic_auth) => basic_auth.description(),
            HttpMatcher::QueryParam(query) => query.description(),
            HttpMatcher::QueryValue(query) => query.description(),
            HttpMatcher::Body(body) => body.description(),
            #[cfg(feature = "json")]
            HttpMatcher::ExactJson(json) => json.description(),
            #[cfg(feature = "json")]
            HttpMatcher::PartialJson(json) => json.description(),
            #[cfg(feature = "xml")]
            HttpMatcher::ExactXml(xml) => xml.description(),
            #[cfg(feature = "xml")]
            HttpMatcher::PartialXml(xml) => xml.description(),
        }
    }
}

impl TypedMatcher<Request> for HttpMatcher {
    fn matcher_type(&self) -> caramelo::MatchType {
        ToHave
    }
}
