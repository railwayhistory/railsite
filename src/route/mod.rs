use htmlfn::core::AttributeValue;
use httools::request::{Request};
use httools::response::{Response, ResponseBuilder};
use crate::page;
use crate::page::Page;
use crate::state::RequestState;


//------------ Sub-modules ---------------------------------------------------

pub mod assets;


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

        match path.next() {
            Some(assets::SEGMENT) => assets::process(path),
            //None => Home::process(state),
            _ => not_found(request, state),
        }
    }

    fn link(state: &RequestState) -> impl AttributeValue + '_ {
        state.url_base()
    }
}


//------------ Home ----------------------------------------------------------

pub struct Home;

impl Home {
    fn process(_state: &RequestState) -> Response {
        unimplemented!()
    }

    pub fn link(state: &RequestState) -> impl AttributeValue + '_ {
        Root::link(state)
    }
}


//------------ not_found -----------------------------------------------------

pub fn not_found(request: Request, state: &RequestState) -> Response {
    page::error::not_found(state, request.path_str()).response(
        ResponseBuilder::new().not_found()
    )
}

