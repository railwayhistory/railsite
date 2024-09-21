use htmlfn::core::Text;
use raildata::document::source;
use crate::state::RequestState;

pub fn title<'a>(
    source: source::Document<'a>, _state: &'a RequestState
) -> impl Text + 'a {
    source.key().as_str()
}

