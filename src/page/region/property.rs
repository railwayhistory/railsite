use htmlfn::core::Content;
use raildata::document::entity;
use crate::i18n;
use crate::state::RequestState;

pub fn status<'a>(
    status: entity::Status,
    state: &'a RequestState
) -> impl Content + 'a {
    use entity::Status::*;

    match status {
        Forming => i18n::term::entity::status::forming(state),
        Open => i18n::term::entity::status::open(state),
        Closed => i18n::term::entity::status::closed(state),
    }
}

