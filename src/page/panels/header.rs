//! The page header panel.

use htmlfn::html;
use htmlfn::html::attr;
use htmlfn::core::Content;
use crate::{i18n, route};
use crate::state::RequestState;


//------------ standard ------------------------------------------------------

pub fn standard(state: &RequestState) -> impl Content + '_ {
    (
        html::p::class("panel-header-brand",
            html::a(
                route::Home::href(state),
                html::img::attrs((
                    attr::src(route::assets::BrandLogo::href(state)),
                    attr::alt(i18n::term::nav::home(state)),
                ))
            )
        ),
        html::form(
            (
                attr::class("panel-header-search"),
                attr::method("get"),
                attr::action(route::aux::Search::href(state)),
            ),
            (
                html::input((
                    attr::name("q"),
                    attr::placeholder(
                        i18n::term::header::search::placeholder(state)
                    ),
                )),
                html::button(
                    "submit", (), i18n::term::header::search::submit(state)
                ),
            )
        ),
    )
}

