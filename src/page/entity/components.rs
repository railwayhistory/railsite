use htmlfn::html;
use htmlfn::core::{Content, Text};
use raildata::document::entity;
use crate::state::RequestState;


#[derive(Clone, Copy)]
pub enum Chapter {
    Overview,
}

impl Chapter {
    pub fn title<'a>(
        self, entity: entity::Document<'a>, state: &'a RequestState
    ) -> impl Text + 'a {
        entity.data().local_short_name(state.lang().into())
    }

    pub fn headline<'a>(
        self, entity: entity::Document<'a>, state: &'a RequestState
    ) -> impl Content + 'a {
        html::h1(entity.data().local_short_name(state.lang().into()))
    }
}

