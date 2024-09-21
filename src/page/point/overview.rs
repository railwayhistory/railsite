use htmlfn::html;
use htmlfn::core::Content;
use htmlfn::utils::iter;
use raildata::document::point;
use crate::i18n;
use crate::page::{frame, snip};
use crate::state::RequestState;
use super::property;

pub fn page<'a>(
    point: point::Document<'a>, state: &'a RequestState
) -> impl frame::Page + 'a {
    frame::standard(state, snip::point::title(point, state), (), (),
        (
            headline(point, state),
            current(point, state),
        )
    )
}

pub fn headline<'a>(
    point: point::Document<'a>, state: &'a RequestState
) -> impl Content + 'a {
    html::h1(snip::point::title(point, state))
}


pub fn current<'a>(
    point: point::Document<'a>,
    state: &'a RequestState
) -> impl Content + 'a {
    let current = &point.meta().current;
    (
        html::h2(i18n::term::point::current(state)),
        html::dl::class("point-current", (
            // Status
            current.status.as_ref().map(|status| {(
                html::dt(i18n::term::point::property::status(state)),
                html::dd(property::status(status.to_value(), state)),
            )}),

            // Name
            current.name.as_ref().map(|name| {(
                html::dt(i18n::term::point::property::name(state)),
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
                html::dt(i18n::term::point::property::short_name(state)),
                html::dd(
                    html::dl::class("local-names",
                        iter(name.iter().map(|(code, name)| {(
                            html::dt(snip::local::opt_local_code(code, state)),
                            html::dd(name.as_value().as_str()),
                        )}))
                    )
                )
            )}),

            // Public Name
            current.public_name.as_ref().map(|name| {(
                html::dt(i18n::term::point::property::public_name(state)),
                html::dd(
                    html::dl::class("local-names",
                        iter(
                            name.iter().map(|name| name.iter()).flatten()
                            .map(|(code, name)| {(
                                html::dt(
                                    snip::local::opt_local_code(code, state)
                                ),
                                html::dd(name.as_value().as_str()),
                            )})
                        )
                    )
                )
            )}),

            // Designation
            current.designation.as_ref().map(|name| {(
                html::dt(i18n::term::point::property::designation(state)),
                html::dd(
                    html::dl::class("local-names",
                        iter(name.iter().map(|(code, name)| {(
                            html::dt(snip::local::opt_local_code(code, state)),
                            html::dd(name.as_value().as_str()),
                        )}))
                    )
                )
            )}),

            // de::name16
            current.de_name16.as_ref().map(|name| {(
                html::dt(i18n::term::point::property::de_name16(state)),
                html::dd(name.as_str()),
            )}),

            // category
            current.category.as_ref().map(|category| {(
                html::dt(i18n::term::point::property::category(state)),
                html::dd(html::ul(
                    iter(category.iter().map(|cat| {
                        html::li(i18n::enums::point::category(
                            cat.into_value(), state
                        ))
                    }))
                )),
            )}),

            // de_rang
            current.de_rang.as_ref().map(|rang| {(
                html::dt(i18n::term::point::property::de_rang(state)),
                html::dd(
                    i18n::enums::point::de_rang(rang.into_value(), state)
                ),
            )}),

            // superior
            current.superior.as_ref().map(|opt| {
                opt.as_ref()
            }).flatten().map(|sup| {(
                html::dt(i18n::term::point::property::superior(state)),
                html::dd(html::ul(
                    iter(sup.iter().map(|link| {
                        html::li(snip::point::link(
                            link.document(state.store()), None, state
                        ))
                    }))
                ))
            )}),

            // codes
            iter(current.codes.iter().map(|(code, value)| {(
                html::dt(
                    i18n::enums::point::code_type(code, state)
                ),
                html::dd(html::ul(
                    iter(value.map(|item| {
                        html::li(item)
                    }))
                ))
            )})),

            // location
            (!current.location.is_empty()).then(|| {(
                html::dt(
                    i18n::term::point::property::location(state)
                ),
                html::dd(html::dl(
                    iter(current.location.iter().map(|(line, loc)| {(
                        html::dt(
                            snip::line::code_link(
                                line.document(state.store()), state
                            )
                        ),
                        html::dd(loc.unwrap_or("–"))
                    )}))
                ))
            )})
        ))
    )
}

