use htmlfn::html;
use htmlfn::core::{Content, Target};
use htmlfn::utils::{either, iter};
use raildata::store::DocumentLink;
use crate::i18n;
use crate::page::{frame, panels, snip};
use crate::route::Href;
use crate::state::RequestState;

//------------ page ----------------------------------------------------------

pub fn page(state: &RequestState) -> impl frame::Page + '_ {
    let term = state.query().get_first("q");
    let items = state.catalogue().search_name(
        term.unwrap_or("")
    ).enumerate().take(51);

    frame::standard(state, i18n::term::aux::search::title(state), (), (), (
        html::h1(i18n::term::aux::search::title(state)),
        html::div::class("aux-search-bar", (
            panels::misc::search_bar(state, term),
        )),
        html::ul::class("aux-search-items",
            iter(items.map(|(idx, (name, link))| {
                item_row(state, idx, name, link)
            }))
        )
    ))
}

fn item_row<'a>(
    state: &'a RequestState,
    idx: usize,
    name: &'a str,
    link: DocumentLink
) -> impl Content + 'a {
    either(
        idx == 50,
        || {
            html::li::class("aux-search-more",
                i18n::term::aux::search::more(state)
            )
        },
        || {
            let doc = link.document(state.store());
            let mut target = Target::new();
            snip::combined::title(doc, state).render_content(&mut target);
            html::li(
                either(
                    name.as_bytes() == target.as_ref(),
                    || {
                        html::a(
                            link.href(state),
                            name
                        )
                    },
                    move || {(
                        name,
                        " → ",
                        html::a(
                            link.href(state),
                            target
                        )
                    )}
                )
            )
        }
    )
}

