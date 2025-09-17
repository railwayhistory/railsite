use htmlfn::html;
use htmlfn::core::Content;
use htmlfn::html::attr;
use htmlfn::utils::iter;
use crate::{i18n, route};
use crate::lang::Lang;
use crate::state::RequestState;

//------------ lang_select ---------------------------------------------------

pub fn lang_select(state: &RequestState) -> impl Content + '_ {
    html::div::class("panel-lang-select", (
        html::p::class("current",
            html::a("#", state.lang().code())
        ),
        html::ul::class("menu",
            iter(Lang::all().map(|lang| {
                html::li::class(
                    if lang == state.lang() { "active" } else { "" },
                    html::a(("?lang=", lang.code()), (
                        html::span::class("code", lang.code()),
                        html::span::class("name", lang.name()),
                    ))
                )
            }))
        ),
    ))
}

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

