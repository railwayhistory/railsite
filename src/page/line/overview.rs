use htmlfn::html;
use htmlfn::core::Content;
use htmlfn::utils::{display, iter, join};
use raildata::document::line;
use crate::i18n;
use crate::page::{frame, snip};
use crate::route::Href;
use crate::state::RequestState;
use super::property;


pub fn page<'a>(
    line: line::Document<'a>, state: &'a RequestState
) -> impl frame::Page + 'a {
    frame::standard(state, snip::line::title(line, state), (), (),
        (
            headline(line, state),
            current(line, state),
            route(line, state),
        )
    )
}


fn headline<'a>(
    line: line::Document<'a>, state: &'a RequestState
) -> impl Content + 'a {
    let jurisdiction = line.data().jurisdiction();
    html::h1::class("line-headline", (
        html::span::class("line-headline-code", (
            line.data().code().as_str(), ". "
        )),
        html::span::class("line-headline-course", (
            line.data().points.first_junction(
                state.store()
            ).data().name_in_jurisdiction(jurisdiction),
            " – ",
            line.data().points.last_junction(
                state.store()
            ).data().name_in_jurisdiction(jurisdiction),
        ))
    ))
}


//------------ current -------------------------------------------------------

pub fn current<'a>(
    line: line::Document<'a>, state: &'a RequestState
) -> impl Content + 'a {
    let jurisdiction = line.data().jurisdiction();
    (
        html::h2(i18n::term::line::current(state)),
        html::dl::class("line-current", (
            // Name
            line.data().current.name.and_then(|name| {(
                html::dt(i18n::term::line::property::name(state)),
                html::dd(current_value(line, name, state, |name| {
                    name.as_ref().and_then(|name| {
                        name.for_language(state.lang().into())
                    }).unwrap_or("–")
                }))
            )}),

            // Course
            //
            html::dt(i18n::term::line::property::course(state)),
            html::dd(
                join(
                    " – ",
                    line.data().points.iter_documents(
                        state.store()
                    ).filter_map(move |point| {
                        point.meta().junction.then(|| {
                            html::a(
                                point.href(state),
                                point.data().name_in_jurisdiction(jurisdiction)
                            )
                        })
                    })
                )
            ),

            // Status
            //
            line.data().current.status.and_then(|status| {(
                html::dt(i18n::term::line::property::status(state)),
                html::dd(current_value(line, status, state, |status| {
                    property::status(*status, state)
                }))
            )}),

            // Category
            //
            line.data().current.category.and_then(|cat| {(
                html::dt(i18n::term::line::property::category(state)),
                html::dd(current_value(line, cat, state, |cat| {
                    join(" ", cat.iter().map(|item| {
                        property::category(*item, state)
                    }))
                }))
            )}),

            // Gauge
            //
            line.data().current.gauge.and_then(|gauge| {(
                html::dt(i18n::term::line::property::gauge(state)),
                html::dd(current_value(line, gauge, state, |gauge| {
                    join("/", gauge.iter().map(|gauge| (
                        display(gauge.gauge()),
                        "\u{202f}mm"
                    )))
                }))
            )}),

            // Rails
            //
            line.data().current.rails.and_then(|rails| {(
                html::dt(i18n::term::line::property::rails(state)),
                html::dd(current_value(line, rails, state, |rails| {
                    display(rails)
                }))
            )}),

            // Tracks
            //
            line.data().current.tracks.and_then(|tracks| {(
                html::dt(i18n::term::line::property::tracks(state)),
                html::dd(current_value(line, tracks, state, |tracks| {
                    display(tracks)
                }))
            )}),

            // Electrified
            //
            line.data().current.electrified.and_then(|el| {(
                html::dt(i18n::term::line::property::electrified(state)),
                html::dd(current_value(line, el, state, |el| {(
                    el.as_ref().map(|el| {
                        Some(join(", ", el.iter().map(|el| {
                            property::electrified(el, state)
                        })))
                    }),
                    el.is_none().then(|| {
                        i18n::term::unknown(state)
                    })
                )}))
            )}),

            // Passenger
            line.data().current.passenger.and_then(|service| {(
                html::dt(i18n::term::line::property::passenger(state)),
                html::dd(current_value(line, service, state, |service| {
                    property::passenger(*service, state)
                }))
            )}),

            // Goods
            line.data().current.goods.and_then(|service| {(
                html::dt(i18n::term::line::property::goods(state)),
                html::dd(current_value(line, service, state, |service| {
                    property::goods(*service, state)
                }))
            )}),

            // Owner
            line.data().current.owner.and_then(|owner| {(
                html::dt(i18n::term::line::property::owner(state)),
                html::dd(current_value(line, owner, state, |owner| {(
                    owner.as_ref().map(|owner| {
                        join(", ", owner.iter().map(|owner| {
                            snip::entity::link(
                                owner.document(state.store()),
                                state
                            )
                        }))
                    }),
                    owner.is_none().then(|| {
                        i18n::term::unknown(state)
                    })
                )}))
            )}),

            // Operator
            line.data().current.operator.and_then(|operator| {(
                html::dt(i18n::term::line::property::operator(state)),
                html::dd(current_value(line, operator, state, |operator| {(
                    operator.as_ref().map(|operator| {
                        join(", ", operator.iter().map(|operator| {
                            snip::entity::link(
                                operator.document(state.store()),
                                state
                            )
                        }))
                    }),
                    operator.is_none().then(|| {
                        i18n::term::unknown(state)
                    })
                )}))
            )}),
        ))
    )
}

fn current_value<'a, T, F, R>(
    line: line::Document<'a>,
    value: &'a line::CurrentValue<T>,
    state: &'a RequestState,
    op: F
) -> impl Content + 'a 
where
    F: Fn(&'a T) -> R + 'a,
    R: Content + 'a
{
    let value = value.as_slice();
    if value.len() == 1 {
        (Some(op(&value[0].1)), None)
    }
    else {
        (None, Some(
            html::dl::class("line-current-sections",
                iter(value.iter().map(move |item| {
                    let jurisdiction = line.data().jurisdiction();
                    (
                        html::dt((
                            snip::point::link(
                                item.0.start_point(line.data(), state.store()),
                                jurisdiction, state
                            ),
                            " – ",
                            snip::point::link(
                                item.0.end_point(line.data(), state.store()),
                                jurisdiction, state
                            ),
                        )),
                        html::dd(op(&item.1))
                    )
                }))
            )
        ))
    }
}


//------------ route ---------------------------------------------------------

pub fn route<'a>(
    line: line::Document<'a>, state: &'a RequestState
) -> impl Content + 'a {
    let link = line.link();
    let jurisdiction = line.data().jurisdiction();
    (
        html::h2(i18n::term::line::route(state)),
        html::table::class("line-route",
            iter(
                line.data().points.iter_documents(
                    state.store()
                ).map(move |point| {
                    html::tr::class(
                        if point.data().is_open() { "" }
                        else { "closed" },
                        (
                            // location
                            html::td(
                                point.data().line_location(
                                    link
                                ).map(|(location, changed)| {
                                    (location, changed.then(|| "*"))
                                })
                            ),

                            // category
                            html::td(
                                point.data().category().map(|(cat, changed)| {(
                                    join(" ", cat.map(|cat| cat.code())),
                                    changed.then(|| "*")
                                )})
                            ),

                            // name
                            html::td(
                                html::a(
                                    point.href(state),
                                    point.data().name_in_jurisdiction(
                                        jurisdiction
                                    )
                                )
                            ),

                            // connections
                            html::td::class("line-route-connections",
                                if point.meta().junction {
                                    Some(html::ul(
                                        iter(
                                            point.xrefs().lines.iter().filter(
                                                move |line| **line != link
                                            ).map(|line| {
                                                html::li(
                                                    snip::line::link(
                                                        line.document(
                                                            state.store()
                                                        ),
                                                        state
                                                    )
                                                )
                                            })
                                        )
                                    ))
                                }
                                else {
                                    None
                                }
                            ),
                        )
                    )
                })
            )
        )
    )
}

