use raildata::document::point;
use crate::i18n;
use crate::state::RequestState;


pub fn status(status: point::Status, state: &RequestState) -> &'static str {
    use point::Status::*;

    match status {
        Planned => i18n::term::point::status::planned(state),
        Construction => i18n::term::point::status::construction(state),
        Open | Reopened => i18n::term::point::status::open(state),
        Suspended => i18n::term::point::status::suspended(state),
        Closed => i18n::term::point::status::closed(state),
    }
}

