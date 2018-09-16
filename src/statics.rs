//! Delivery of static content.

use hyper::{Body, Method, Request, Response, StatusCode};

const TEXT_CSS: &str = "text/css";
const TEXT_JAVASCRIPT: &str = "text/javascript";

macro_rules! statics {
    ( $request:expr, $( ($path:expr, $mime:expr), )* ) => {{
        match $request.uri().path() {
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


pub fn serve_statics(request: &Request<Body>) -> Option<Response<Body>> {
    statics!(request,
        ("style.css", TEXT_CSS),
        ("js/bootstrap.min.js", TEXT_JAVASCRIPT),
        ("js/jquery.min.js", TEXT_JAVASCRIPT),
        ("js/popper.min.js", TEXT_JAVASCRIPT),
    )
}


pub fn serve_str(
    request: &Request<Body>,
    content: &'static str,
    ctype: &'static str,
) -> Response<Body> {
    if let &Method::GET = request.method() {
        Response::builder()
            .header("Content-Type", ctype)
            .body(content.into())
            .unwrap()
    }
    else {
        Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .header("Allow", "GET")
            .body("Method Not Allowed".into())
            .unwrap()
    }
}

