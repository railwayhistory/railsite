//! Routes for static assets.

use htmlfn::core::AttributeValue;
use httools::request::PathIter;
use httools::response::{ContentType, Response, ResponseBuilder};
use crate::state::RequestState;
use super::RouteError;

pub(super) const SEGMENT: &'static str = "static";

macro_rules! assets {
    ( $(
        struct $type:ident:
        ($path:expr, $mime:expr)
    ),* $( , )? ) => {
        $(
            pub struct $type;

            impl $type {
                pub fn href(
                    state: &RequestState
                ) -> impl AttributeValue + '_ {
                    (
                        super::Root::href(state),
                        SEGMENT,
                        concat!("/", $path)
                    )
                }
            }
        )*

        pub(super) fn process(
            path: PathIter
        ) -> Result<Response, RouteError> {
            match path.remaining() {
                $(
                    $path => {
                        Ok(ResponseBuilder::new().content_type($mime).body(
                            include_bytes!(
                                concat!("../../static/", $path)
                            ).as_ref()
                        ))
                    }
                )*
                "style.css" => {
                    Ok(
                        ResponseBuilder::new().content_type(
                            ContentType::CSS
                        ).body(
                            grass::include!("style/style.scss")
                        )
                    )
                }
                _ => Err(RouteError::NotFound)
            }
        }
    }
}

pub struct StyleCss;

impl StyleCss {
    pub fn href(
        state: &RequestState
    ) -> impl AttributeValue + '_ {
        (
            super::Root::href(state),
            SEGMENT,
            "/style.css",
        )
    }
}

assets!(
    struct BrandLogo: ("img/logo64-blue.svg", ContentType::SVG),
    struct FrontLogo: ("img/logo500-beige.svg", ContentType::SVG),
    struct BasicScript: ("js/basic.js", ContentType::JS),
);

