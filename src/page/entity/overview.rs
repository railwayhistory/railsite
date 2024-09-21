use htmlfn::html;
use htmlfn::core::Content;
use htmlfn::utils::{iter, join};
use raildata::document::entity;
use crate::i18n;
use crate::page::{frame, snip};
use crate::state::RequestState;
use super::components::Chapter;
use super::property;

const CHAPTER: Chapter = Chapter::Overview;

pub fn page<'a>(
    entity: entity::Document<'a>, state: &'a RequestState
) -> impl frame::Page + 'a {
    frame::standard(state, CHAPTER.title(entity, state), (), (), (
        CHAPTER.headline(entity, state),
        current(entity, state),
    ))
}

pub fn current<'a>(
    entity: entity::Document<'a>,
    state: &'a RequestState
) -> impl Content + 'a {
    let current = &entity.meta().current;
    (
        html::h2(i18n::term::entity::properties(state)),
        html::dl::class("entity-current", (
            // Name
            current.name.as_ref().map(|name| {(
                html::dt(i18n::term::entity::property::name(state)),
                html::dd(
                    html::dl::class("local-names",
                        iter(name.iter().map(|(code, name)| {(
                            html::dt(snip::local::opt_local_code(code, state)),
                            html::dd(name.as_value().as_str()),
                        )}))
                    )
                )
            )}),

            // Short Name
            current.short_name.as_ref().map(|name| {(
                html::dt(i18n::term::entity::property::short_name(state)),
                html::dd(
                    html::dl::class("local-names",
                        iter(name.iter().map(|(code, name)| {(
                            html::dt(snip::local::opt_local_code(code, state)),
                            html::dd(name.as_value().as_str()),
                        )}))
                    )
                )
            )}),

            // Superior
            current.superior.as_ref().map(|superior| {(
                html::dt(i18n::term::entity::property::superior(state)),
                html::dd(
                    join(", ", superior.iter().map(|superior| {
                        snip::entity::link(
                            superior.document(state.store()), state
                        )
                    }))
                )
            )}),

            // Domicile
            current.domicile.as_ref().map(|domicile| {(
                html::dt(i18n::term::entity::property::domicile(state)),
                html::dd(
                    join(", ", domicile.iter().map(|domicile| {
                        snip::entity::link(
                            domicile.document(state.store()), state
                        )
                    }))
                )
            )}),

            // Owner
            current.owner.as_ref().map(|owner| {(
                html::dt(i18n::term::entity::property::owner(state)),
                html::dd(
                    join(", ", owner.iter().map(|owner| {
                        snip::entity::link(
                            owner.document(state.store()), state
                        )
                    }))
                )
            )}),

            // Status
            current.status.as_ref().map(|status| {(
                html::dt(i18n::term::entity::property::status(state)),
                html::dd(property::status(status.to_value(), state)),
            )}),

            // Successor
            current.successor.as_ref().map(|successor| {(
                html::dt(i18n::term::entity::property::successor(state)),
                html::dd(
                    snip::entity::link(
                        successor.document(state.store()), state
                    )
                )
            )})
        ))
    )
}
