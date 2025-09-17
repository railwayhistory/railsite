use htmlfn::core::AttributeValue;
use httools::request::PathIter;
use httools::response::Response;
use raildata::document::combined::{LineDocument, LineLink};
use raildata::store::DocumentLink;
use crate::page;
use crate::page::Page;
use crate::state::RequestState;
use super::{Href, RouteError};


//------------ process -------------------------------------------------------

pub(super) fn process(
    line: LineDocument, mut path: PathIter, state: &RequestState
) -> Result<Response, RouteError> {
    if let Some(_sub) = path.next() {
        return Err(RouteError::NotFound)
    }
    Ok(page::line::overview(line, state).ok(state))
}


//------------ LineLink ------------------------------------------------------

impl Href for LineLink {
    fn href(self, state: &RequestState) -> impl AttributeValue + '_ {
        DocumentLink::from(self).href(state)
    }
}


//------------ LineDocument --------------------------------------------------

impl Href for LineDocument<'_> {
    fn href(self, state: &RequestState) -> impl AttributeValue + '_ {
        self.link().href(state)
    }
}

