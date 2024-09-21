use htmlfn::html;
use htmlfn::core::{Content, Text};
use raildata::document::entity;
use crate::{i18n, route};
use crate::state::RequestState;


#[derive(Clone, Copy)]
pub enum Chapter {
    Overview,
    Lines,
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
        (
            html::h1(entity.data().local_short_name(state.lang().into())),
            html::div::class("standard-subpage-nav",
                html::ul((
                    html::li::class(
                        matches!(self, Self::Overview).then(|| "active"),
                        html::a(
                            route::region::Overview::href(entity, state),
                            i18n::term::entity::subpage::overview(state)
                        )
                    ),
                    html::li::class(
                        matches!(self, Self::Lines).then(|| "active"),
                        html::a(
                            route::region::Lines::href(entity, state),
                            i18n::term::entity::subpage::lines(state)
                        )
                    ),
                ))
            )
        )
    }
}

