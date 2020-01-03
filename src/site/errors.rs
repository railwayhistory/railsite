use hyper::StatusCode;
use crate::html;
use crate::http::{Request, Response};

pub fn method_not_allowed(request: &Request) -> Response {
    request.respond(
        StatusCode::METHOD_NOT_ALLOWED,
        html::errors::method_not_allowed(request).into()
    )
}

pub fn not_found(request: &Request) -> Response {
    request.respond(
        StatusCode::NOT_FOUND,
        html::errors::not_found(request).into()
    )
}

