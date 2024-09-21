use htmlfn::core::Text;
use raildata::document::structure;
use crate::state::RequestState;

pub fn title<'a>(
    structure: structure::Document<'a>, state: &'a RequestState
) -> impl Text + 'a {
    let lang = state.lang().into();
    for event in structure.data().events.iter().rev() {
        if let Some(name) = event.name.as_ref() {
            if let Some(name) = name.for_language(lang) {
                return name
            }
        }
    }
    structure.key().as_str()
}

