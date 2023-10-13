//! Routes for static assets.

use htmlfn::core::AttributeValue;
use httools::request::PathIter;
use httools::response::{ContentType, Response, ResponseBuilder};
use crate::state::RequestState;

pub(super) const SEGMENT: &'static str = "static";

macro_rules! assets {
    ( $(
        struct $type:ident:
        ($path:expr, $mime:expr)
    ),* $( , )? ) => {
        $(
            pub struct $type;

            impl $type {
                pub fn link(
                    state: &RequestState
                ) -> impl AttributeValue + '_ {
                    (
                        super::Root::link(state),
                        SEGMENT,
                        concat!("/", $path)
                    )
                }
            }
        )*

        pub(super) fn process(path: PathIter) -> Response {
            match path.remaining() {
                $(
                    $path => {
                        ResponseBuilder::new().content_type($mime).body(
                            include_bytes!(
                                concat!("../../static/", $path)
                            ).as_ref()
                        )
                    }
                )*
                "style.css" => {
                    ResponseBuilder::new().content_type(ContentType::CSS).body(
                        grass::include!("style/style.scss")
                    )
                }
                _ => unimplemented!()
            }
        }
    }
}

pub struct StyleCss;

impl StyleCss {
    pub fn link(
        state: &RequestState
    ) -> impl AttributeValue + '_ {
        (
            super::Root::link(state),
            SEGMENT,
            "/style.css",
        )
    }
}

assets!(
    struct BrandLogo: ("img/logo64-blue.svg", ContentType::SVG),
);

