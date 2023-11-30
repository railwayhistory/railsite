use htmlfn::html;
use htmlfn::core::{Content, Text};
use raildata::document::source;
use crate::state::RequestState;

pub fn title<'a>(
    source: source::Document<'a>, state: &'a RequestState
) -> impl Text + 'a {
    source.data().name(state.lang().into())
}

pub fn headline<'a>(
    source: source::Document<'a>, state: &'a RequestState
) -> impl Content + 'a {
    html::h1(source.data().name(state.lang().into()))
}

