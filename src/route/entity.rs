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
    entity: entity::Document, path: PathIter, state: &RequestState
) -> Result<Response, RouteError> {
    match entity.data().subtype.into_value() {
        entity::Subtype::Country | entity::Subtype::Region => {
            super::region::process(entity, path, state)
        }
        _ => process_entity(entity, path, state),
    }
}


fn process_entity(
    entity: entity::Document, mut path: PathIter, state: &RequestState
) -> Result<Response, RouteError> {
    let sub = match path.next() {
        Some(sub) => sub,
        None => return Ok(Overview::process(entity, state)),
    };
    if let Some(_subsub) = path.next() {
        return Err(RouteError::NotFound)
    }
    match sub {
        _ => Err(RouteError::NotFound)
    }
}


//------------ Href impls ----------------------------------------------------

impl Href for entity::Link {
    fn href(self, state: &RequestState) -> impl AttributeValue + '_ {
        DocumentLink::from(self).href(state)
    }
}

impl<'a> Href for entity::Document<'a> {
    fn href(self, state: &RequestState) -> impl AttributeValue + '_ {
        self.link().href(state)
    }
}


//------------ Overview ------------------------------------------------------

pub struct Overview;

impl Overview {
    fn process(entity: entity::Document, state: &RequestState) -> Response {
        page::entity::overview(entity, state).ok(state)
    }

    pub fn href<'a>(
        entity: entity::Document<'a>, state: &'a RequestState
    ) -> impl AttributeValue + 'a {
        entity.href(state)
    }
}

