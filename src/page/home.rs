use htmlfn::html;
use htmlfn::html::attr;
use htmlfn::core::Content;
use crate::{i18n, route};
use crate::page::{frame, panels};
use crate::state::RequestState;


//------------ standard ------------------------------------------------------

pub fn standard(state: &RequestState) -> impl frame::Page + '_ {
    frame::basic(state, "railwayhistory.org", (), (), (
        banner(state),
        main(state),
    ))
}

//------------ banner --------------------------------------------------------

/// The big brand banner shown at the top of the home page.
pub fn banner(state: &RequestState) -> impl Content + '_ {
    html::div::class("home-banner",
        html::h1((
            html::img::attrs((
                attr::src(route::assets::FrontLogo::href(state)),
                attr::alt(i18n::term::home::logo(state)),
            )),
            "railwayhistory.org"),
        )
    )
}


//------------ main ----------------------------------------------------------

/// The main area of the home page with category buttons and search bar.
pub fn main(state: &RequestState) -> impl Content + '_ {
    html::div::class("home-main", (
        html::div::class("home-main-intro",
            html::p((
                i18n::term::home::intro::text(state),
                " ",
                html::a::class("more", "#",
                    i18n::term::home::intro::more(state)
                ),
            ))
        ),
        html::div::class("home-main-buttons", (
            html::ul::class("home-main-buttons-chapters", (
                html::li(html::a(
                    route::aux::Countries::href(state),
                    i18n::term::home::button::countries(state)
                )),
                html::li(html::a(
                    "https://map.railwayhistory.org/",
                    i18n::term::home::button::map(state)
                )),
            )),
            html::div::class("home-main-lang", 
                panels::misc::lang_select(state)
            ),
        )),
        html::div::class("home-main-search", (
            panels::misc::search_bar(state, None), 
            html::div::id("home-main-search-results", ()),
        )),
    ))
}

