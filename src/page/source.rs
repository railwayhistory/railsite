use htmlfn::html;
use raildata::document::source;
use crate::page::{frame, snip};
use crate::state::RequestState;

pub fn overview<'a>(
    source: source::Document<'a>, state: &'a RequestState
) -> impl frame::Page + 'a {
    frame::standard(state,
        snip::source::title(source, state), (), (),
        html::h1(
            snip::source::title(source, state)
        )
    )
}

