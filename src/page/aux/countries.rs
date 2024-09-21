use htmlfn::html;
use htmlfn::core::Content;
use htmlfn::utils::iter;
use crate::i18n;
use crate::page::frame;
use crate::route::Href;
use crate::state::RequestState;

//------------ page ----------------------------------------------------------

pub fn page(state: &RequestState) -> impl frame::Page + '_ {
    frame::standard(state, i18n::term::aux::countries::title(state), (), (), (
        html::h1(i18n::term::aux::countries::title(state)),
        index(state),
    ))
}

fn index(state: &RequestState) -> impl Content + '_ {
    let mut countries = state.catalogue().countries.values().map(|link| {
        (link, link.document(state.store()))
    }).collect::<Vec<_>>();
    countries.sort_by_key(|(_, doc)| {
        doc.data().local_short_name(state.lang().into())
    });
    html::ul(
        iter(countries.into_iter().map(|(link, doc)| {
            html::li(
                html::a(
                    link.href(state),
                    doc.data().local_short_name(state.lang().into())
                )
            )
        }))
    )
}

