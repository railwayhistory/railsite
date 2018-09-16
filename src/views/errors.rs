use htmlfn::Content;
use hyper::{Body, Request};
use super::core::other;

pub fn not_found<'a>(request: &'a Request<Body>) -> impl Content + 'a {
    other("en",
        "404 Not Found",
        elements!(
            h1 {
                "404 Not Found"
            }
            p {
                tt { request.uri().path() }
                " not found on this server."
            }
        )
    )
}

