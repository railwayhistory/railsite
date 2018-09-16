//! Error pages

use htmlfn::Content;
use hyper::StatusCode;
use hyper::{Body, Request, Response};
use ::views::errors::not_found;


pub fn serve_404(
    request: Request<Body>
) -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(not_found(&request).into_string().into())
        .unwrap()
}

