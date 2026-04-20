use std::error::Error;

use googletest::{expect_that, gtest, prelude::eq};
use http::{Method, StatusCode};

use crate::mock::{MethodExt, Mock, StatusCodeExt};

#[gtest]
fn test_mock_request() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        Method::GET
            .has()
            .path("/test")
            .will_return(
                StatusCode::OK
                    .respond()
                    .with_body(b"teste"),
            ),
    );

    expect_that!(
        mock.request()
            .method(),
        eq(Method::GET)
    );

    expect_that!(
        mock.request()
            .path(),
        eq("/test")
    );

    Ok(())
}
