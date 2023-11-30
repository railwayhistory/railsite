use htmlfn::core::AttributeValue;
use httools::request::{Request};
use httools::response::Response;
use crate::page;
use crate::page::Page;
use crate::state::RequestState;


//------------ Sub-modules ---------------------------------------------------

pub mod assets;
pub mod document;
pub mod entity;
pub mod line;
pub mod point;
pub mod source;
pub mod structure;


//------------ Root ----------------------------------------------------------

pub struct Root;

impl Root {
    pub fn process(request: Request, state: &RequestState) -> Response {
        let path = match request.path() {
            Ok(path) => path,
            Err(_) => {
                unimplemented!()
            }
        };
        let mut path = path.iter();

        let res = match path.next() {
            Some(assets::SEGMENT) => assets::process(path),
            Some(document::SEGMENT) => document::process(path, state),
            None => Home::process(state),
            _ => Err(RouteError::NotFound),
        };

        match res {
            Ok(res) => res,
            Err(RouteError::NotFound) => {
                page::error::not_found(state, request.path_str()).response(
                    state.response().not_found()
                )
            }
        }
    }

    fn href(state: &RequestState) -> impl AttributeValue + '_ {
        state.url_base()
    }
}


//------------ Home ----------------------------------------------------------

pub struct Home;

impl Home {
    fn process(state: &RequestState) -> Result<Response, RouteError> {
        Ok(page::home::standard(state).ok(state))
    }

    pub fn href(state: &RequestState) -> impl AttributeValue + '_ {
        Root::href(state)
    }
}


//------------ Href ----------------------------------------------------------

pub trait Href {
    fn href(self, state: &RequestState) -> impl AttributeValue + '_;
}


//------------ RouteError ----------------------------------------------------

enum RouteError {
    NotFound,
}

