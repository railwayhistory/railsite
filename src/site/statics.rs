use hyper::{Body, StatusCode};
use crate::html::target::RenderText;
use crate::http::{GetRequest, Response};
use super::Site;

macro_rules! statics {
    ( $( ( $( : $name:ident : )* $path:expr => $mime:expr ), )* ) => {
        impl Site {
            pub fn process_statics(
                &self, mut request: GetRequest
            ) -> Result<Response, GetRequest> {
                if request.path().segment() != "static" {
                    return Err(request)
                }
                request.path_mut().next();
                match request.path().remaining() {
                    $(
                        $path => {
                            Ok(request.respond_raw(
                                StatusCode::OK,
                                $mime,
                                Body::from(include_bytes!(
                                    concat!("../../static/", $path)
                                ).as_ref())
                            ))
                        }
                    )*
                    _ => Ok(self.not_found(request))
                }
            }

            $(
                $(
                    pub fn $name<'s>(&'s self) -> impl RenderText + 's {
                        self.link(concat!("/static/", $path))
                    }
                )*
            )*
        }
    }
}

statics!{
    (:link_style_css: "style.css" => "text/css"),
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
    (:link_bootstrap_js: "js/bootstrap.min.js" => "application/javascript"),
    (:link_jquery_js: "js/jquery.min.js" => "application/javascript"),
    (:link_popper_js: "js/popper.min.js" => "application/javascript"),
    (:link_openlayers_js: "js/ol.js" => "application/javascript"),
    (:link_openlayers_css: "css/ol.css" => "text/css"),
    (:link_skeleton_js: "js/skeleton.js" => "application/javascript"),
    (:link_home_js: "js/home.js" => "application/javascript"),
    (:link_map_js: "js/map.js" => "application/javascript"),
    (:link_brand_logo: "img/logo64-beige.svg" => "image/svg+xml"),
}

