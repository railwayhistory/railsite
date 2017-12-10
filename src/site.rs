use futures::future::Future;
use hyper;
use hyper::server::{Request, Response, Service};
use super::errors::serve_404;
use super::statics::serve_statics;

pub struct Railsite;

impl Service for Railsite {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, request: Request) -> Self::Future {
        if let Some(response) = serve_statics(&request) {
            return response
        }
        serve_404(request)
    }
}

