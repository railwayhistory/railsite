use raildata::document::structure;
use crate::panel;
use crate::state::RequestState;
use super::frame;

pub fn overview<'a>(
    structure: structure::Document<'a>, state: &'a RequestState
) -> impl frame::Page + 'a {
    frame::standard(state, panel::structure::title(structure, state), (), (),
        (
            panel::structure::headline(structure, state),
        )
    )
}


