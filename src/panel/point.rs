use htmlfn::html;
use htmlfn::core::{Content, Text};
use raildata::document::point;
use raildata::types::local::CountryCode;
use crate::route::Href;
use crate::state::RequestState;

pub fn title<'a>(
    point: point::Document<'a>, state: &'a RequestState
) -> impl Text + 'a {
    point.data().name(state.lang().into())
}

pub fn headline<'a>(
    point: point::Document<'a>, state: &'a RequestState
) -> impl Content + 'a {
    html::h1(point.data().name(state.lang().into()))
}

pub fn link<'a>(
    point: point::Document<'a>,
    jurisdiction: Option<CountryCode>,
    state: &'a RequestState
) -> impl Content + 'a {
    html::a(
        point.href(state),
        point.data().name_in_jurisdiction(jurisdiction)
    )
}

