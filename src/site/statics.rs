use horrorshow::{/*html, */Template};
use hyper::{Body, StatusCode};
use crate::http::{Response, Request};
use super::errors;

macro_rules! statics {
    ( $( ( $( : $name:ident : )* $path:expr => $mime:expr ), )* ) => {
        pub fn process(
            request: Request
        ) -> Result<Response, Request> {
            if request.path().segment() != "static" {
                return Err(request)
            }
            match request.path().remaining() {
                $(
                    concat!("/", $path) => {
                        if request.is_get() {
                            Ok(request.respond_raw(
                                StatusCode::OK,
                                $mime,
                                Body::from(include_bytes!(
                                    concat!("../../static/", $path)
                                ).as_ref())
                            ))
                        }
                        else {
                            Ok(errors::method_not_allowed(&request))
                        }
                    }
                )*
                _ => Ok(errors::not_found(&request))
            }
        }

        $(
            $(
                #[allow(dead_code)]
                pub fn $name(_request: &Request) -> impl Template {
                    $path
                    /*
                    request.link(
                        html! {
                            : "static/";
                            : $path
                        }
                    )
                    */
                }
            )*
        )*
    }
}

statics!{
    (:style_css: "style.css" => "text/css"),
    ("fonts/fa-brands-400.eot" => "application/vnd.ms-fontobject"),
    ("fonts/fa-brands-400.svg" => "image/svg+xml"),
    ("fonts/fa-brands-400.ttf" => "font/ttf"),
    ("fonts/fa-brands-400.woff" => "font/woff"),
    ("fonts/fa-brands-400.woff2" => "font/woff2"),
    ("fonts/fa-regular-400.eot" => "application/vnd.ms-fontobject"),
    ("fonts/fa-regular-400.svg" => "image/svg+xml"),
    ("fonts/fa-regular-400.ttf" => "font/ttf"),
    ("fonts/fa-regular-400.woff" => "font/woff"),
    ("fonts/fa-regular-400.woff2" => "font/woff2"),
    ("fonts/fa-solid-900.eot" => "application/vnd.ms-fontobject"),
    ("fonts/fa-solid-900.svg" => "image/svg+xml"),
    ("fonts/fa-solid-900.ttf" => "font/ttf"),
    ("fonts/fa-solid-900.woff" => "font/woff"),
    ("fonts/fa-solid-900.woff2" => "font/woff2"),
    (:bootstrap_js: "js/bootstrap.min.js" => "application/javascript"),
    (:jquery_js: "js/jquery.min.js" => "application/javascript"),
    (:popper_js: "js/popper.min.js" => "application/javascript"),
}

