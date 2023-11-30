use htmlfn::core::AttributeValue;
use httools::request::PathIter;
use httools::response::Response;
use raildata::document::entity;
use raildata::store::DocumentLink;
use crate::page;
use crate::page::Page;
use crate::state::RequestState;
use super::{Href, RouteError};


//------------ process -------------------------------------------------------

pub(super) fn process(
    entity: entity::Document, mut path: PathIter, state: &RequestState
) -> Result<Response, RouteError> {
    if let Some(_sub) = path.next() {
        return Err(RouteError::NotFound)
    }
    Ok(page::entity::overview(entity, state).ok(state))
}


//------------ entity::Link --------------------------------------------------

impl Href for entity::Link {
    fn href(self, state: &RequestState) -> impl AttributeValue + '_ {
        DocumentLink::from(self).href(state)
    }
}


//------------ Document ------------------------------------------------------

impl<'a> Href for entity::Document<'a> {
    fn href(self, state: &RequestState) -> impl AttributeValue + '_ {
        self.data().link().href(state)
    }
}

