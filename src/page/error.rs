use htmlfn::html;
use crate::state::RequestState;
use super::frame;
use super::frame::Page;


pub fn not_found<'a>(state: &'a RequestState, path: &'a str) -> impl Page + 'a {
    frame::standard(state, "Not Found", (), (),
        super::frame::Nav::Other,
        (
            html::h1("Not Found"),
            html::p((
                "The page at ",
                html::tt((
                    state.url_base(),
                    path,
                )),
                " was not found."
            ))
        )
    )
}

