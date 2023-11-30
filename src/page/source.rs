use raildata::document::source;
use crate::panel;
use crate::state::RequestState;
use super::frame;

pub fn overview<'a>(
    source: source::Document<'a>, state: &'a RequestState
) -> impl frame::Page + 'a {
    frame::standard(state, panel::source::title(source, state), (), (),
        (
            panel::source::headline(source, state),
        )
    )
}

