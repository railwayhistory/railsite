use htmlfn::core::AttributeValue;
use httools::request::PathIter;
use httools::response::Response;
use raildata::document::Document;
use raildata::store::DocumentLink;
use crate::state::RequestState;
use super::{Href, RouteError};


//------------ process -------------------------------------------------------

pub const SEGMENT: &'static str = "key";

pub(super) fn process(
    mut path: PathIter, state: &RequestState
) -> Result<Response, RouteError> {
    let key = match path.next() {
        Some(key) => key,
        None => return Err(RouteError::NotFound)
    };
    let doc = match state.store().get(key) {
        Some(doc) => doc.document(state.store()),
        None => return Err(RouteError::NotFound)
    };
    match doc {
        Document::Line(line) => super::line::process(line, path, state),
        Document::Entity(entity) => super::entity::process(entity, path, state),
        Document::Point(point) => super::point::process(point, path, state),
        Document::Source(src) => super::source::process(src, path, state),
        Document::Structure(s) => super::structure::process(s, path, state),
        _ => Err(RouteError::NotFound)
    }
}


//------------ DocumentLink --------------------------------------------------

impl Href for DocumentLink {
    fn href(self, state: &RequestState) -> impl AttributeValue + '_ {
        (
            super::Root::href(state),
            SEGMENT, "/",
            self.document(state.store()).key().as_str(),
        )
    }
}

