use htmlfn::html;
use crate::state::RequestState;
use super::frame;


//------------ standard ------------------------------------------------------

pub fn standard(state: &RequestState) -> impl frame::Page + '_ {
    frame::basic(state, "railwayhistory.org", (), (),
        html::p("home")
    )
}

