use raildata::document::point;
use crate::panel;
use crate::state::RequestState;
use super::frame;

pub fn overview<'a>(
    point: point::Document<'a>, state: &'a RequestState
) -> impl frame::Page + 'a {
    frame::standard(state, panel::point::title(point, state), (), (),
        (
            panel::point::headline(point, state),
        )
    )
}

