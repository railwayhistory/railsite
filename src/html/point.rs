
use std::collections::HashMap;
use raildata::document::point::{Event, Point, Record, ServiceSet};
use raildata::types::{CountryCode, EventDate, LocalText};
use crate::site::Site;
use crate::site::point::Part;
use super::{cmark, common, i18n, line, skeleton, source};
use super::components::{property_view, stub_badge};
use super::source::SourceList;
use super::target::{Content, Target, Text, empty};


//------------ details -------------------------------------------------------

pub fn details(site: &Site, point: &Point, part: Part) -> Target {
    skeleton::multipart_sheet(
        site,
        point.name(site.lang().into()), // title
        empty, // head
        skeleton::Nav::Other,
        |cont| {
            cont.h1().text(point.name(site.lang().into()))
        },
        |nav| {
            nav.item(
                site.link_point_part(point, Part::Overview),
                matches!(part, Part::Overview),
                |cont| cont.text(i18n::point::term_part_overview)
            );
            nav.item(
                site.link_point_part(point, Part::Events),
                matches!(part, Part::Events),
                |cont| cont.text(i18n::point::term_part_events)
            );
            nav.item(
                site.link_point_part(point, Part::Records),
                matches!(part, Part::Records),
                |cont| cont.text(i18n::point::term_part_records)
            );
        },
        |cont| {
            match part {
                Part::Overview => overview_part(cont, point, site),
                Part::Events => events_part(cont, point, site),
                Part::Records => records_part(cont, point, site),
            }
        },
        empty, // scripts
    )
}


//------------ overview_part -------------------------------------------------

fn overview_part(cont: &mut Content, point: &Point, site: &Site) {
    if point.progress().is_stub() {
        stub_badge(cont);
    }

    if point.subtype.is_post() {
        overview_post(cont, point, site)
    }
    else {
        overview_non_post(cont, point, site)
    }
}

fn overview_post(cont: &mut Content, point: &Point, site: &Site) {
    let mut designation = false;

    property_view(cont, |view| {
        // status
        view.property(
            |name| name.text(i18n::point::term_overview_status),
            |values| {
                let mut any = false;
                for event in &point.events {
                    if let Some(status) = event.properties.status {
                        values.dated(
                            |date| date.text(&event.date),
                            |value| value.text(status.into_value()),
                        );
                        any = true;
                    }
                }
                if !any {
                    values.value(|cont| {
                        cont.i().text(i18n::common::term_not_available)
                    });
                }
            }
        );

        // name
        view.property(
            |name| name.text(i18n::point::term_overview_name),
            |values| {
                let mut any = false;
                for event in &point.events {
                    if let Some(name) = event.properties.name.as_ref() {
                        values.dated(
                            |date| date.text(&event.date),
                            |value| local_name(value, name),
                        );
                        any = true;
                    }
                    if let Some(name) = event.properties.designation.as_ref() {
                        values.dated(
                            |date| date.text(&event.date),
                            |value| {
                                local_name(value, name);
                                value.text("*")
                            }
                        );
                        any = true;
                        designation = true;
                    }
                }
                if !any {
                    values.value(|cont| {
                        cont.i().text(i18n::common::term_not_available)
                    });
                }
            }
        );

        // category
        view.property(
            |name| name.text(i18n::point::term_overview_category),
            |values| {
                let mut any = false;
                for event in &point.events {
                    if let Some(category) = event.properties.category.as_ref() {
                        values.dated(
                            |date| date.text(&event.date),
                            |value| {
                                for cat in category.iter() {
                                    value.text(cat.code());
                                    value.text(" ");
                                }
                            }
                        );
                        any = true;
                    }
                }
                if !any {
                    values.value(|cont| {
                        cont.i().text(i18n::common::term_not_available)
                    });
                }
            }
        );

        // location
        view.property(
            |name| name.text(i18n::point::term_overview_location),
            |values| values.value(|cont| overview_location(cont, point, site)),
        );
    });

    if designation {
        cont.p().text(i18n::point::term_overview_designation_footnote);
    }
}

fn overview_non_post(cont: &mut Content, point: &Point, site: &Site) {
    let mut designation = false;

    property_view(cont, |view| {
        // subtype
        view.property(
            |name| name.text(i18n::point::term_overview_subtype),
            |values| {
                values.value(|cont| cont.text(point.subtype.into_value()))
            }
        );

        // name
        view.property(
            |name| name.text(i18n::point::term_overview_name),
            |values| {
                let mut any = false;
                for event in &point.events {
                    if let Some(name) = event.properties.name.as_ref() {
                        values.dated(
                            |date| date.text(&event.date),
                            |value| local_name(value, name),
                        );
                        any = true;
                    }
                    if let Some(name) = event.properties.designation.as_ref() {
                        values.dated(
                            |date| date.text(&event.date),
                            |value| {
                                local_name(value, name);
                                value.text("*")
                            }
                        );
                        any = true;
                        designation = true;
                    }
                }
                if !any {
                    values.value(|cont| {
                        cont.i().text(i18n::common::term_not_available)
                    });
                }
            }
        );

        // location
        view.property(
            |name| name.text(i18n::point::term_overview_location),
            |values| values.value(|cont| overview_location(cont, point, site)),
        );
    });

    if designation {
        cont.p().text(i18n::point::term_overview_designation_footnote);
    }
}

fn local_name(cont: &mut Content, name: &LocalText) {
    for (code, name) in name.iter() {
        if let Some(code) = code {
            cont.i().content(|cont| {
                cont.text(code.into_value());
                cont.text(": ");
            })
        }
        cont.text(name);
        cont.br();
    }
}

fn overview_location(cont: &mut Content, point: &Point, site: &Site) {
    let mut lines: HashMap<_, Vec<_>> = {
        site.catalogue().point_connections.get_lines(
            point.link()
        ).iter().map(|line| (*line, Vec::new())).collect()
    };

    for event in &point.events {
        for (line, loc) in event.properties.location.iter() {
            lines.entry(line).or_default().push((&event.date, loc))
        }
    }

    let mut lines: Vec<_> = lines.into_iter().map(|(line, locs)| {
        static DEFAULT_DATE: EventDate = EventDate::new();
        (
            line.follow(site.library()),
            if locs.is_empty() {
                vec![(&DEFAULT_DATE, None)] 
            }
            else {
                locs
            }
        )
    }).collect();
    lines.sort_unstable_by_key(|item| item.0.key());

    if lines.is_empty() {
        cont.i().text(i18n::common::term_not_available)
    }

    cont.table().class("point-location-table").content(|cont| {
        for (line, locs) in lines {
            let mut len = Some(locs.len());
            for (date, loc) in locs {
                cont.tr().content(|cont| {
                    if let Some(len) = len.take() {
                        cont.th()
                            .attr("scope", "row")
                            .attr("rowspan", format_args!("{}", len))
                        .content(|cont| line::link(cont, line, site));
                    }

                    cont.td()
                        .class("point-location-date")
                    .content(|cont| cont.text(date));
                    cont.td().class("point-location-location").content(|cont| {
                        match loc {
                            Some(loc) => cont.text(loc),
                            None => cont.text("\u{2014}"),
                        }
                    })
                });
            }
        }
    })
}


//------------ events_part ---------------------------------------------------

fn events_part(cont: &mut Content, point: &Point, site: &Site) {
    let mut sources = SourceList::new("event-source");
    cont.table().class("details-events-table").content(|cont| {
        for item in &point.events {
            event(cont, item, site, &mut sources);
        }
    });
    sources.render(cont, site);
}

fn event(
    cont: &mut Content,
    event: &Event,
    site: &Site,
    sources: &mut SourceList
) {
    cont.tr().content(|cont| {
        cont.td().class("details-events-date").content(|cont| {
            if event.date.is_empty() {
                cont.text("\u{2014}");
            }
            else {
                cont.text(&event.date);
            }
        });
        cont.td().class("details-events-facts").content(|cont| {
            cont.ul().content(|cont| {
                // status
                if let Some(status) = event.properties.status {
                    cont.li().content(|cont| {
                        i18n::point::event_status(cont, status.into_value())
                    })
                }

                // split_from
                if let Some(link) = event.split_from {
                    cont.li().content(|cont| {
                        i18n::point::event_split_from(
                            cont,
                            link.follow(site.library()),
                            site
                        )
                    })
                }

                // merged
                if let Some(link) = event.merged {
                    cont.li().content(|cont| {
                        i18n::point::event_merged(
                            cont,
                            link.follow(site.library()),
                            site
                        )
                    })
                }

                // name
                if let Some(name) = event.properties.name.as_ref() {
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
                if let Some(name) = event.properties.short_name.as_ref() {
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

                // public_name
                if let Some(name) = event.properties.public_name.as_ref() {
                    for (lang, name) in name.iter().flatten() {
                        cont.li().content(|cont| {
                            if let Some(lang) = lang {
                                i18n::common::event_local_public_name(
                                    cont, lang.into_value(), name.as_value()
                                )
                            }
                            else {
                                i18n::common::event_public_name(
                                    cont, name.as_value()
                                )
                            }
                        });
                    }
                }

                // designation
                if let Some(des) = event.properties.designation.as_ref() {
                    for (lang, des) in des {
                        cont.li().content(|cont| {
                            if let Some(lang) = lang {
                                i18n::common::event_local_designation(
                                    cont, lang.into_value(), des.as_value()
                                )
                            }
                            else {
                                i18n::common::event_designation(
                                    cont, des.as_value()
                                )
                            }
                        });
                    }
                }

                // de_name16
                if let Some(name) = event.properties.de_name16.as_ref() {
                    cont.li().content(|cont| {
                        i18n::point::event_de_name16(cont, name.as_ref())
                    })
                }

                // category
                if let Some(category) = event.properties.category.as_ref() {
                    cont.li().content(|cont| {
                        i18n::point::event_category(
                            cont, category.iter().map(|item| item.into_value())
                        );
                    })
                }

                // de_rang
                if let Some(rang) = event.properties.de_rang {
                    cont.li().content(|cont| {
                        i18n::point::event_de_rang(cont, rang.into_value())
                    })
                }

                // superior
                if let Some(sup) = event.properties.superior.as_ref() {
                    cont.li().content(|cont| {
                        match sup.len() {
                            0 => {
                                i18n::point::event_no_superior(cont)
                            }
                            1 => {
                                i18n::point::event_one_superior(
                                    cont,
                                    sup.first().unwrap().follow(
                                        site.library()
                                    ),
                                    site
                                )
                            }
                            _ => {
                                i18n::point::event_many_superiors(
                                    cont,
                                    sup.iter().map(|item| {
                                        item.follow(site.library())
                                    }),
                                    site
                                )
                            }
                        }
                    });
                }

                // codes
                for (code_type, codes) in event.properties.codes.iter() {
                    cont.li().content(|cont| {
                        i18n::point::event_code(cont, code_type, codes);
                    });
                }

                // location
                if !event.properties.location.is_empty() {
                    cont.li().content(|cont| {
                        i18n::point::event_location(
                            cont, event.properties.location.iter(), site
                        )
                    })
                }

                // staff
                if let Some(staff) = event.properties.staff {
                    cont.li().content(|cont| {
                        cont.text(staff)
                    })
                }

                let service = ServiceSet::from(&event.properties);
                if service.is_some() {
                    cont.li().content(|cont| {
                        i18n::point::event_service(cont, service);
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


//------------ records_part---------------------------------------------------

fn records_part(cont: &mut Content, point: &Point, site: &Site) {
    for item in &point.records {
        cont.div().class("core-subpart details-events-facts").content(|cont| {
            record(cont, item, site);
        })
    }
}

fn record(
    cont: &mut Content,
    event: &Record,
    site: &Site,
) {
    let document = event.document.follow(site.library());
    cont.h2().content(|cont| {
        cont.a().href(site.link_source(document)).content(|cont| {
            source::short_title(cont, document, site)
        });
    });
    cont.ul().content(|cont| {
        // status
        if let Some(status) = event.properties.status {
            cont.li().content(|cont| {
                i18n::point::event_status(cont, status.into_value())
            })
        }

        // name
        if let Some(name) = event.properties.name.as_ref() {
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
        if let Some(name) = event.properties.short_name.as_ref() {
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

        // public_name
        if let Some(name) = event.properties.public_name.as_ref() {
            for (lang, name) in name.iter().flatten() {
                cont.li().content(|cont| {
                    if let Some(lang) = lang {
                        i18n::common::event_local_public_name(
                            cont, lang.into_value(), name.as_value()
                        )
                    }
                    else {
                        i18n::common::event_public_name(
                            cont, name.as_value()
                        )
                    }
                });
            }
        }

        // designation
        if let Some(des) = event.properties.designation.as_ref() {
            for (lang, des) in des {
                cont.li().content(|cont| {
                    if let Some(lang) = lang {
                        i18n::common::event_local_designation(
                            cont, lang.into_value(), des.as_value()
                        )
                    }
                    else {
                        i18n::common::event_designation(
                            cont, des.as_value()
                        )
                    }
                });
            }
        }

        // de_name16
        if let Some(name) = event.properties.de_name16.as_ref() {
            cont.li().content(|cont| {
                i18n::point::event_de_name16(cont, name.as_ref())
            })
        }

        // category
        if let Some(category) = event.properties.category.as_ref() {
            cont.li().content(|cont| {
                i18n::point::event_category(
                    cont, category.iter().map(|item| item.into_value())
                );
            })
        }

        // de_rang
        if let Some(rang) = event.properties.de_rang {
            cont.li().content(|cont| {
                i18n::point::event_de_rang(cont, rang.into_value())
            })
        }

        // superior
        if let Some(sup) = event.properties.superior.as_ref() {
            cont.li().content(|cont| {
                match sup.len() {
                    0 => {
                        i18n::point::event_no_superior(cont)
                    }
                    1 => {
                        i18n::point::event_one_superior(
                            cont,
                            sup.first().unwrap().follow(
                                site.library()
                            ),
                            site
                        )
                    }
                    _ => {
                        i18n::point::event_many_superiors(
                            cont,
                            sup.iter().map(|item| {
                                item.follow(site.library())
                            }),
                            site
                        )
                    }
                }
            });
        }

        // codes
        for (code_type, codes) in event.properties.codes.iter() {
            cont.li().content(|cont| {
                i18n::point::event_code(cont, code_type, codes);
            });
        }

        // location
        if !event.properties.location.is_empty() {
            cont.li().content(|cont| {
                i18n::point::event_location(
                    cont, event.properties.location.iter(), site
                )
            })
        }

        // staff
        if let Some(staff) = event.properties.staff {
            cont.li().content(|cont| {
                cont.text(staff)
            })
        }

        let service = ServiceSet::from(&event.properties);
        if service.is_some() {
            cont.li().content(|cont| {
                i18n::point::event_service(cont, service);
            })
        }
    });
}



//------------ Public Components ---------------------------------------------

pub fn link(cont: &mut Content, point: &Point, site: &Site) {
    cont.a().href(site.link_point(point)).content(|cont| {
        cont.text(point.name(cont.lang().into()))
    })
}

pub fn link_in_jurisdiction(
    cont: &mut Content, point: &Point, jur: Option<CountryCode>, site: &Site
) {
    cont.a().href(site.link_point(point)).content(|cont| {
        cont.text(point.name_in_jurisdiction(jur))
    });
}

