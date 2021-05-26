
use raildata::document::{Organization, OrganizationLink};
use raildata::document::organization::Event;
use raildata::types::EventDate;
use crate::site::Site;
use crate::site::organization::Part;
use super::{cmark, common, i18n, line, skeleton};
use super::components::{property_view, stub_badge, PropertyViewItem};
use super::source::SourceList;
use super::target::{Content, Target, Text, empty};


//------------ details -------------------------------------------------------

pub fn details(site: &Site, org: &Organization, part: Part) -> Target {
    skeleton::multipart_sheet(
        site,
        org.local_short_name(site.lang().into()), // title
        empty, // head
        skeleton::Nav::Other,
        |cont| { // headline
            cont.h1().content(|cont| {
                cont.text(org.local_short_name(site.lang().into()))
            });
        },
        |nav| {
            nav.item(
                site.link_organization_part(org, Part::Overview),
                matches!(part, Part::Overview),
                |cont| cont.text(i18n::org::term_part_overview)
            );
            nav.item(
                site.link_organization_part(org, Part::Events),
                matches!(part, Part::Events),
                |cont| cont.text(i18n::org::term_part_events)
            );
            nav.item(
                site.link_organization_part(org, Part::Property),
                matches!(part, Part::Property),
                |cont| cont.text(i18n::org::term_part_property)
            );
            if Part::Lines.is_valid(org) {
                nav.item(
                    site.link_organization_part(org, Part::Lines),
                    matches!(part, Part::Lines),
                    |cont| cont.text(i18n::org::term_part_lines)
                );
            }
        },
        |cont| {
            match part {
                Part::Overview => overview_part(cont, org, site),
                Part::Events => events_part(cont, org, site),
                Part::Property => property_part(cont, org, site),
                Part::Lines => lines_part(cont, org, site),
            }
        },
        empty, // scripts
    )
}


//------------ overview_part -------------------------------------------------

fn overview_part(cont: &mut Content, org: &Organization, site: &Site) {
    if org.progress().is_stub() {
        stub_badge(cont);
    }
    property_view(cont, |view| {
        view.property(
            |name| name.text(i18n::org::term_overview_status),
            |values| overview_status(values, org)
        );

        // name
        view.property(
            |name| name.text(i18n::org::term_overview_name),
            |values| overview_name(values, org)
        );

        // superior
        if org.events.iter().any(|item| item.superior.is_some()) {
            view.property(
                |name| name.text(i18n::org::term_overview_superior),
                |values| overview_superior(values, org, site)
            );
        }
    });
}

fn overview_status(values: &mut PropertyViewItem, org: &Organization) {
    let mut something = false;
    for event in org.events.iter() {
        if let Some(status) = event.status {
            something = true;
            values.dated(
                |date| {
                    if !event.date.is_empty() {
                        date.text(&event.date);
                    }
                },
                |value| value.text(status.into_value())
            );
        }
    }
    if !something {
        values.value(|cont| {
            cont.i().text(i18n::org::term_not_available)
        });
    }
}

fn overview_name(values: &mut PropertyViewItem, org: &Organization) {
    let mut something = false;
    for event in org.events.iter() {
        if let Some(name) = event.name.as_ref().and_then(|name| {
            name.for_language(values.lang().into())
        }) {
            something = true;
            values.dated(
                |date| {
                    if !event.date.is_empty() {
                        date.text(&event.date);
                    }
                },
                |cont| cont.text(name)
            );
        }
    }
    if !something {
        values.value(|cont| cont.i().text(i18n::org::term_not_available));
    }
}

fn overview_superior(
    values: &mut PropertyViewItem, org: &Organization, site: &Site
) {
    for event in org.events.iter() {
        if let Some(org) = event.superior {
            values.dated(
                |date| {
                    if !event.date.is_empty() {
                        date.text(&event.date);
                    }
                },
                |cont| {
                    link_at(
                        cont, org.into_value(), &event.date, site
                    );
                }
            );
        }
    }
}


//------------ events_part ---------------------------------------------------

fn events_part(cont: &mut Content, org: &Organization, site: &Site) {
    let mut sources = SourceList::new("event-source");
    cont.table().class("details-events-table").content(|cont| {
        for item in &org.events {
            event(cont, item, org, site, &mut sources);
        }
    });
    sources.render(cont, site);
}


fn event(
    cont: &mut Content,
    event: &Event,
    org: &Organization,
    site: &Site,
    sources: &mut SourceList
) {
    cont.tr().content(|cont| {
        cont.td().class("details-events-date").text(&event.date);
        cont.td().class("details-events-facts").content(|cont| {
            cont.ul().content(|cont| {
                // status
                if let Some(status) = event.status {
                    cont.li().content(|cont| {
                        i18n::org::event_status(
                            cont,
                            status.into_value(),
                            org.subtype.into_value()
                        )
                    })
                }

                // name
                if let Some(name) = event.name.as_ref() {
                    for (lang, name) in name {
                        cont.li().content(|cont| {
                            if let Some(lang) = lang {
                                i18n::common::event_local_name(
                                    cont, lang.into_value(), name.as_value()
                                )
                            }
                            else {
                                i18n::common::event_name(
                                    cont, name.as_value()
                                )
                            }
                        });
                    }
                }

                // short_name
                if let Some(name) = event.short_name.as_ref() {
                    for (lang, name) in name {
                        cont.li().content(|cont| {
                            if let Some(lang) = lang {
                                i18n::common::event_local_short_name(
                                    cont, lang.into_value(), name.as_value()
                                )
                            }
                            else {
                                i18n::common::event_short_name(
                                    cont, name.as_value()
                                )
                            }
                        });
                    }
                }

                // superior
                if let Some(superior) = event.superior.as_ref() {
                    cont.li().content(|cont| {
                        i18n::org::event_superior(
                            cont, superior.into_value(), &event.date, site
                        );
                    })
                }

                // domicile
                if !event.domicile.is_empty() {
                    if org.subtype.is_geographical() {
                        cont.li().content(|cont| {
                            i18n::org::event_capital(
                                cont, &event.domicile, &event.date, site
                            )
                        });
                    }
                    else {
                        cont.li().content(|cont| {
                            i18n::org::event_domicile(
                                cont, &event.domicile, &event.date, site
                            )
                        });
                    }
                }

                // owner
                if let Some(owner) = event.owner.as_ref() {
                    cont.li().content(|cont| {
                        i18n::org::event_owner(cont, owner, &event.date, site)
                    });
                }

                // property
                if let Some(property) = event.property.as_ref() {
                    cont.li().content(|cont| {
                        i18n::org::event_property(
                            cont, property, &event.date, site
                        );
                    })
                }

                // successor
                if let Some(successor) = event.successor.as_ref() {
                    cont.li().content(|cont| {
                        i18n::org::event_successor(
                            cont, successor.into_value(), &event.date, site
                        )
                    })
                }
            });

            // basis
            if !event.basis.is_empty() {
                cont.ul().content(|cont| {
                    for basis in &event.basis {
                        cont.li().content(|cont| {
                            common::event_basis(
                                 cont, basis, site, sources
                            );
                        });
                    }
                })
            }

            // note
            if let Some(note) = event.note.as_ref().and_then(|text| {
                text.for_language(site.lang().into())
            }) {
                cont.div().class("details-events-note").content(|cont| {
                    cmark::render(cont, note, site)
                })
            }
        });
        cont.td().class("details-events-source").content(|cont| {
            for link in event.document.iter().chain(event.source.iter()) {
                let num = sources.add(link.into_value());
                cont.a()
                    .href(|text: &mut Text| write!(text, "#source-{}", num))
                .text(|text: &mut Text| write!(text, "[{}]", num));
            }
        })
    });
}


//------------ property_part -------------------------------------------------

fn property_part(cont: &mut Content, org: &Organization, site: &Site) {
    let org_link = site.library().get(org.key()).unwrap().into();
    let lines = site.catalogue().property_lines.by_link(org_link);

    if lines.is_empty() {
        cont.p().content(|cont| {
            cont.i().text(i18n::org::term_property_empty)
        });
        return
    }

    cont.table().class("details-index-table").content(|cont| {
        cont.thead().content(|cont| {
            cont.tr().content(|cont| {
                cont.th().class("here-code").attr("scope", "col").text(
                    i18n::org::term_property_line
                );
                cont.th().attr("scope", "col").text(
                    i18n::org::term_property_course
                );
                cont.th().class("here-not-ticked").attr("scope", "col").text(
                    i18n::org::term_property_owner
                );
                cont.th().class("here-not-ticked").attr("scope", "col").text(
                    i18n::org::term_property_operator
                );
            });
        });
        cont.tbody().content(|cont| {
            for property in lines {
                let line = property.line.follow(site.library());
                cont.tr().content(|cont| {
                    cont.th().class("here-code").attr("scope", "row")
                    .content(|cont| {
                        line::link(cont, line, site);
                    });
                    cont.td().content(|cont| {
                        line::title(cont, line, site)
                    });
                    if let Some(true) = property.owned {
                        cont.td().class("here-ticked").text("\u{2713}");
                    }
                    else {
                        cont.td().class("here-not-ticked").text("");
                    }
                    if let Some(true) = property.operated {
                        cont.td().class("here-ticked").text("\u{2713}");
                    }
                    else {
                        cont.td().class("here-not-ticked").text("");
                    }
                });
            }
        });
    });
}


//------------ lines_part ----------------------------------------------------

fn lines_part(cont: &mut Content, org: &Organization, site: &Site) {
    let org_link = site.library().get(org.key()).unwrap().into();
    let lines = site.catalogue().country_lines.by_link(org_link);

    cont.table().class("details-index-table").content(|cont| {
        cont.thead().content(|cont| {
            cont.tr().content(|cont| {
                cont.th().class("here-code").attr("scope", "col").text(
                    i18n::org::term_property_line
                );
                cont.th().attr("scope", "col").text(
                    i18n::org::term_property_course
                );
            });
        });
        cont.tbody().content(|cont| {
            for link in lines {
                let line = link.follow(site.library());
                cont.tr().content(|cont| {
                    cont.th().class("here-code").attr("scope", "row")
                    .content(|cont| {
                        line::link(cont, line, site);
                    });
                    cont.td().content(|cont| {
                        line::title(cont, line, site)
                    });
                });
            }
        });
    });
}


//------------ OrganizationLink ----------------------------------------------

pub fn link(site: &Site, link: OrganizationLink, cont: &mut Content) {
    let org = link.follow(site.library());
    cont.a().href(site.link_organization(org)).content(|cont| {
        cont.text(org.local_short_name(site.lang().into()))
    })
}

pub fn link_at(
    cont: &mut Content,
    org: OrganizationLink,
    date: &EventDate,
    site: &Site,
) {
    let org = org.follow(site.library());
    cont.a().href(site.link_organization(org)).content(|cont| {
        cont.text(org.historic_short_name(cont.lang().into(), date))
    })
}

pub fn title(cont: &mut Content, link: OrganizationLink, site: &Site) {
    let org = link.follow(site.library());
    cont.text(org.local_short_name(site.lang().into()))
}

