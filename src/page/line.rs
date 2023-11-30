use raildata::document::combined::LineDocument;
use crate::panel;
use crate::state::RequestState;
use super::frame;


pub fn overview<'a>(
    line: LineDocument<'a>, state: &'a RequestState
) -> impl frame::Page + 'a {
    frame::standard(state, panel::line::title(line, state), (), (),
        (
            panel::line::headline(line, state),
            panel::line::current(line, state),
            panel::line::route(line, state),
        )
    )
}
