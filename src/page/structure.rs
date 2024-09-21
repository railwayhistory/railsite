use htmlfn::html;
use raildata::document::structure;
use crate::page::{frame, snip};
use crate::state::RequestState;

pub fn overview<'a>(
    structure: structure::Document<'a>, state: &'a RequestState
) -> impl frame::Page + 'a {
    frame::standard(state,
        snip::structure::title(structure, state), (), (),
        html::h1(
            snip::structure::title(structure, state),
        )
    )
}


