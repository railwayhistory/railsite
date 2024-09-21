use htmlfn::core::AttributeValue;
use httools::request::PathIter;
use httools::response::Response;
use crate::page;
use crate::page::Page;
use crate::state::RequestState;
use super::RouteError;


//------------ process -------------------------------------------------------

pub(super) fn process(
    segment: &str, mut path: PathIter, state: &RequestState
) -> Result<Response, RouteError> {
    if path.next().is_some() {
        return Err(RouteError::NotFound)
    }
    match segment {
        Countries::SEGMENT => Ok(Countries::process(state)),
        Search::SEGMENT => Ok(Search::process(state)),
        _ => Err(RouteError::NotFound)
    }
}


//------------ Countries -----------------------------------------------------

pub struct Countries;

impl Countries {
    const SEGMENT: &'static str = "countries";

    fn process(state: &RequestState) -> Response {
        page::aux::countries(state).ok(state)
    }

    pub fn href(state: &RequestState) -> impl AttributeValue + '_ {
        (super::Root::href(state), Self::SEGMENT)
    }
}


//------------ Search --------------------------------------------------------

pub struct Search;

impl Search {
    const SEGMENT: &'static str = "search";

    fn process(state: &RequestState) -> Response {
        page::aux::search(state).ok(state)
    }

    pub fn href(state: &RequestState) -> impl AttributeValue + '_ {
        (super::Root::href(state), Self::SEGMENT)
    }
}

