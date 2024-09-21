use htmlfn::core::Text;
use raildata::document::path;
use crate::state::RequestState;

pub fn title<'a>(
    path: path::Document<'a>, _state: &'a RequestState
) -> impl Text + 'a {
    path.key().as_str()
}

