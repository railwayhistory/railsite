use hyper::StatusCode;
use crate::html;
use crate::http::{Request, Response};
use super::Site;


//------------ Site ----------------------------------------------------------

impl Site {
    pub fn method_not_allowed(&self, request: Request) -> Response {
        html::errors::method_not_allowed(
            self, request.method(), request.path().full()
        ).into_response(StatusCode::METHOD_NOT_ALLOWED)
    }

    pub fn not_found(&self, request: impl Into<Request>) -> Response {
        let request = request.into();
        html::errors::not_found(
            self, request.path().full()
        ).into_response(StatusCode::NOT_FOUND)
    }
}

