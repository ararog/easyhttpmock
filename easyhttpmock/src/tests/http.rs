use caramelo::expect;
use http::{header::CONTENT_TYPE, Method, Uri};

use crate::{
    matchers::{header, header_value, method, path},
    mock::Request,
};

#[test]
fn test_path_matcher() {
    let request = Request::get(Uri::from_static("/api/users"))
        .empty()
        .unwrap();

    expect(request).to_have(path(r"^/api/.*$"));
}

#[test]
#[should_panic = "Expected Request { method: GET, uri: /users, version: HTTP/1.1, headers: {}, body: None } to have path matching Regex(\"^/api/.*$\")"]
fn test_path_matcher_panic() {
    let request = Request::get(Uri::from_static("/users"))
        .empty()
        .unwrap();

    expect(request).to_have(path(r"^/api/.*$"));
}

#[test]
fn test_method_matcher() {
    let request = Request::get(Uri::from_static("/api/users"))
        .empty()
        .unwrap();

    expect(request).to_have(method(Method::GET));
}

#[test]
#[should_panic = "Expected Request { method: POST, uri: /api/users, version: HTTP/1.1, headers: {}, body: None } to have method matching GET"]
fn test_method_matcher_panic() {
    let request = Request::post(Uri::from_static("/api/users"))
        .empty()
        .unwrap();

    expect(request).to_have(method("GET"));
}

#[test]
#[should_panic = "Expected Request { method: POST, uri: /api/users, version: HTTP/1.1, headers: {}, body: None } to have method matching POST and path matching Regex(\"^/api/posts$\")"]
fn test_method_and_path_matcher_panic() {
    let request = Request::post(Uri::from_static("/api/users"))
        .empty()
        .unwrap();

    expect(request)
        .to_have(method(Method::POST))
        .and(path(r"^/api/posts$"));
}

#[test]
fn test_header_matcher() {
    let request = Request::get(Uri::from_static("/api/users"))
        .header("content-type", "application/json")
        .empty()
        .unwrap();

    expect(request).to_have(header(CONTENT_TYPE));
}

#[test]
#[should_panic = "Expected Request { method: GET, uri: /api/users, version: HTTP/1.1, headers: {\"content-store\": \"application/json\"}, body: None } to have header matching content-type"]
fn test_header_matcher_failure() {
    let request = Request::get(Uri::from_static("/api/users"))
        .header("content-store", "application/json")
        .empty()
        .unwrap();

    expect(request).to_have(header(CONTENT_TYPE));
}

#[test]
fn test_header_value_regex() {
    let request = Request::get(Uri::from_static("/api/users"))
        .header("content-type", "application/json")
        .empty()
        .unwrap();

    expect(request).to_have(header_value("content-type", r"^application/.*"));
}

#[test]
#[should_panic = "Expected Request { method: GET, uri: /api/users, version: HTTP/1.1, headers: {\"content-type\": \"application/json\"}, body: None } to have header content-type with value matching Regex(\"^text/.*\")"]
fn test_header_value_regex_failure() {
    let request = Request::get(Uri::from_static("/api/users"))
        .header("content-type", "application/json")
        .empty()
        .unwrap();

    expect(request).to_have(header_value("content-type", r"^text/.*"));
}
