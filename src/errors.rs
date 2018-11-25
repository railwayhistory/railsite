//! Error pages

use htmlfn::Content;
use hyper;
use ::core::{Request, Response};
use ::views::errors::not_found;


pub fn serve_404(request: Request) -> Response {
    hyper::Response::builder()
        .status(hyper::StatusCode::NOT_FOUND)
        .body(not_found(&request).into_string().into())
        .unwrap()
}



pub enum Error {
    NotFound,
}

impl Error {
    pub fn handle(self, request: Request) -> Response {
        match self {
            Error::NotFound => serve_404(request)
        }
    }
}
