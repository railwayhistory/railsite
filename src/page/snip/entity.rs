use htmlfn::html;
use htmlfn::core::{Content, Text};
use raildata::document::entity;
use crate::route::Href;
use crate::state::RequestState;

pub fn title<'a>(
    entity: entity::Document<'a>, state: &'a RequestState
) -> impl Text + 'a {
    entity.data().local_short_name(state.lang().into());
}

pub fn link<'a>(
    entity: entity::Document<'a>,
    state: &'a RequestState
) -> impl Content + 'a {
    html::a(
        entity.href(state),
        entity.data().local_short_name(state.lang().into())
    )
}

