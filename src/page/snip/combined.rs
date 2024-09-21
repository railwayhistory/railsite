use htmlfn::core::Text;
use raildata::document::Document;
use crate::state::RequestState;


pub fn title<'a>(
    document: Document<'a>, state: &'a RequestState
) -> impl Text + 'a {(
    document.try_as_line().map(|inner| super::line::title(inner, state)),
    document.try_as_entity().map(|inner| super::entity::title(inner, state)),
    document.try_as_path().map(|inner| super::path::title(inner, state)),
    document.try_as_point().map(|inner| super::point::title(inner, state)),
    document.try_as_source().map(|inner| super::source::title(inner, state)),
    document.try_as_structure().map(|inner| {
        super::structure::title(inner, state)
    }),
)}

