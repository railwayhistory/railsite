//! Delivery of static content.

use futures::future;
use futures::future::Future;
use hyper::mime;
use hyper::{Error, Method, StatusCode};
use hyper::header::{Allow, ContentLength, ContentType};
use hyper::server::{Request, Response};

macro_rules! statics {
    ( $request:expr, $( ($path:expr, $mime:expr), )* ) => {{
        match $request.path() {
            $(
                concat!("/static/", $path) => {
                    Some(serve_str($request,
                                   include_str!(concat!("../static/", $path)),
                                   $mime))
                }
            )*
            _ => None
        }
    }}
}


pub fn serve_statics(request: &Request)
                     -> Option<Box<Future<Item=Response, Error=Error>>> {
    statics!(request,
        ("style.css", mime::TEXT_CSS),
        ("js/bootstrap.min.js", mime::TEXT_JAVASCRIPT),
        ("js/jquery.min.js", mime::TEXT_JAVASCRIPT),
        ("js/popper.min.js", mime::TEXT_JAVASCRIPT),
    )
}


pub fn serve_str(request: &Request, content: &'static str, ctype: mime::Mime)
                 -> Box<Future<Item=Response, Error=Error>> {
    if let &Method::Get = request.method() {
        Box::new(future::ok(
            Response::new()
                .with_header(ContentLength(content.len() as u64))
                .with_header(ContentType(ctype))
                .with_body(content)
        ))
    }
    else {
        Box::new(future::ok(
            Response::new()
                .with_status(StatusCode::MethodNotAllowed)
                .with_header(Allow(vec![Method::Get]))
                .with_header(ContentLength("Method Not Allowed".len() as u64))
                .with_body("Method Not Allowed")
        ))
    }
}
