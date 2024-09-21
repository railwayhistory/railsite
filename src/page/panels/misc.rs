use htmlfn::html;
use htmlfn::core::Content;
use htmlfn::html::attr;
use crate::{i18n, route};
use crate::state::RequestState;

//------------ search_bar ----------------------------------------------------

pub fn search_bar<'a>(
    state: &'a RequestState, term: Option<&'a str>
) -> impl Content + 'a {
    html::form(
        (
            attr::class("panel-search-bar"),
            attr::method("get"),
            attr::action(route::aux::Search::href(state)),
        ),
        (
            html::input((
                attr::name("q"),
                attr::placeholder(
                    i18n::term::panel::search::placeholder(state)
                ),
                term.map(|term| attr::value(term)),
            )),
            html::button(
                "submit", (), i18n::term::panel::search::submit(state)
            ),
        )
    )
}

