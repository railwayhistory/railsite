use raildata::document::entity;
use crate::panel;
use crate::state::RequestState;
use super::frame;

pub fn overview<'a>(
    entity: entity::Document<'a>, state: &'a RequestState
) -> impl frame::Page + 'a {
    frame::standard(state, panel::entity::title(entity, state), (), (),
        (
            panel::entity::headline(entity, state),
        )
    )
}

