//! Error pages

use futures::future;
use futures::future::Future;
use hyper::{Error, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Request, Response};
use ::views::errors::not_found;


pub fn serve_404(request: Request)
                 -> Box<Future<Item=Response, Error=Error>> {
    let body = not_found(&request);
    Box::new(future::ok(
        Response::new()
            .with_status(StatusCode::NotFound)
            .with_header(ContentLength(body.len() as u64))
            .with_body(body)
    ))
}

