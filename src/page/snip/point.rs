use htmlfn::html;
use htmlfn::core::{Content, Text};
use raildata::document::point;
use raildata::types::local::CountryCode;
use crate::route::Href;
use crate::state::RequestState;

pub fn title<'a>(
    point: point::Document<'a>, state: &'a RequestState
) -> impl Text + 'a {
    let lang = state.lang().into();
    for event in point.data().events.iter().rev() {
        if let Some(name) = event.name(lang) {
            return name
        }
        if let Some(name) = event.designation(lang) {
            return name
        }
    }
    point.key().as_str()
}

pub fn link<'a>(
    point: point::Document<'a>,
    jurisdiction: Option<CountryCode>,
    state: &'a RequestState
) -> impl Content + 'a {
    html::a(
        point.href(state),
        point.data().name_in_jurisdiction(jurisdiction)
    )
}

