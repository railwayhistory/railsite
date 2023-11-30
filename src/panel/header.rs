//! The page header panel.

use htmlfn::html;
use htmlfn::html::attr;
use htmlfn::core::Content;
use crate::{i18n, route};
use crate::state::RequestState;


//------------ standard ------------------------------------------------------

pub fn standard(state: &RequestState) -> impl Content + '_ {
    (
        html::a::class(
            "panel-header-brand", route::Home::href(state),
            html::img::attrs((
                attr::src(route::assets::BrandLogo::href(state)),
                attr::alt(i18n::term::nav::home(state)),
            ))
        ),
    )
}

