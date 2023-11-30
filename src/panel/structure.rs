use htmlfn::html;
use htmlfn::core::{Content, Text};
use raildata::document::structure;
use crate::state::RequestState;

pub fn title<'a>(
    structure: structure::Document<'a>, state: &'a RequestState
) -> impl Text + 'a {
    structure.data().name(state.lang().into())
}

pub fn headline<'a>(
    structure: structure::Document<'a>, state: &'a RequestState
) -> impl Content + 'a {
    html::h1(structure.data().name(state.lang().into()))
}


