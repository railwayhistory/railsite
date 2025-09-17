use htmlfn::core::AttributeValue;
use httools::request::PathIter;
use httools::response::Response;
use raildata::document::source;
use raildata::store::DocumentLink;
use crate::page;
use crate::page::Page;
use crate::state::RequestState;
use super::{Href, RouteError};


//------------ process -------------------------------------------------------

pub(super) fn process(
    source: source::Document, mut path: PathIter, state: &RequestState
) -> Result<Response, RouteError> {
    if let Some(_sub) = path.next() {
        return Err(RouteError::NotFound)
    }
    Ok(page::source::overview(source, state).ok(state))
}


//------------ Link ----------------------------------------------------------

impl Href for source::Link {
    fn href(self, state: &RequestState) -> impl AttributeValue + '_ {
        DocumentLink::from(self).href(state)
    }
}


//------------ Document ------------------------------------------------------

impl<'a> Href for source::Document<'a> {
    fn href(self, state: &RequestState) -> impl AttributeValue + '_ {
        self.link().href(state)
    }
}

