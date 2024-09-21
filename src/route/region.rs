use htmlfn::core::AttributeValue;
use httools::request::PathIter;
use httools::response::Response;
use raildata::document::entity;
use crate::page;
use crate::page::Page;
use crate::state::RequestState;
use super::{Href, RouteError};


//------------ process -------------------------------------------------------

pub(super) fn process(
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
        Lines::SEGMENT => Ok(Lines::process(entity, state)),
        _ => Err(RouteError::NotFound)
    }
}


//------------ Overview ------------------------------------------------------

pub struct Overview;

impl Overview {
    fn process(entity: entity::Document, state: &RequestState) -> Response {
        page::region::overview(entity, state).ok(state)
    }

    pub fn href<'a>(
        entity: entity::Document<'a>, state: &'a RequestState
    ) -> impl AttributeValue + 'a {
        entity.href(state)
    }
}


//------------ Lines ---------------------------------------------------------

pub struct Lines;

impl Lines {
    const SEGMENT: &'static str = "lines";

    fn process(entity: entity::Document, state: &RequestState) -> Response {
        page::region::lines(entity, state).ok(state)
    }

    pub fn href<'a>(
        entity: entity::Document<'a>, state: &'a RequestState
    ) -> impl AttributeValue + 'a {
        (entity.href(state), "/", Self::SEGMENT)
    }
}

