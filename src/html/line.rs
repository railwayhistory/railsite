/// Pages for line documents.

use raildata::document::{Line, PointLink, SourceLink};
use raildata::document::line::{
    CurrentValue, Event, Properties, Record, Section, Status,
};
use raildata::types::{EventDate, List};
use crate::site::Site;
use crate::site::line::Part;
use super::{common, i18n, organization, point, skeleton, source};
use super::components::{PropertyTable, property_table, stub_badge};
use super::source::SourceList;
use super::target::{Content, RenderText, Target, Text, empty};


//------------ details -------------------------------------------------------

/// Renders the details page for a line document.
pub fn details(site: &Site, line: &Line, part: Part) -> Target {
    skeleton::multipart_sheet(
        site,
        line.code(),
        empty, // head
        skeleton::Nav::Other,
        |cont: &mut Content| { // headline
            cont.h1().text(line.code());
            cont.p().text(|cont: &mut Text| {
                let jurisdiction = line.jurisdiction();
                let first = line.points.first().unwrap().follow(
                    site.library()
                );
                let last = line.points.last().unwrap().follow(
                    site.library()
                );
                write!(
                    cont, "{}\u{a0}– {}",
                    first.name_in_jurisdiction(jurisdiction),
                    last.name_in_jurisdiction(jurisdiction)
                );

            });
        },
        |nav| {
            nav.item(
                site.link_line_part(&line, Part::Overview),
                matches!(part, Part::Overview),
                |cont| cont.text(i18n::line::term_section_overview)
            );
            nav.item(
                site.link_line_part(&line, Part::Route),
                matches!(part, Part::Route),
                |cont| cont.text(i18n::line::term_section_route)
            );
            nav.item(
                site.link_line_part(&line, Part::Events),
                matches!(part, Part::Events),
                |cont| cont.text(i18n::line::term_section_events)
            );
            nav.item(
                site.link_line_part(&line, Part::Records),
                matches!(part, Part::Records),
                |cont| cont.text(i18n::line::term_section_records)
            );
        },
        |cont| {
            match part {
                Part::Overview => overview_section(site, line, cont),
                Part::Route => route_section(site, line, cont),
                Part::Events => events_section(site, line, cont),
                Part::Records=> records_section(site, line, cont),
            }
        },
        empty, // scripts
    )
}


//------------ overview_section ----------------------------------------------

fn overview_section(site: &Site, line: &Line, cont: &mut Content) {
    if line.progress().is_stub() {
        stub_badge(cont);
    }
    property_table(cont, |ref mut table| {
        // course
        overview_course(site, line, table);

        // status
        overview_part(
            site, line, table,
            i18n::line::term_overview_status,
            &line.current.status,
            |status, cont| cont.text(*status)
        );

        // category
        opt_overview_part(
            site, line, table,
            i18n::line::term_overview_category,
            &line.current.category,
            |category, cont| {
                for cat in category.iter() {
                    cont.text(*cat);
                    cont.text(" ");
                }
            }
        );

        // gauge
        opt_overview_part(
            site, line, table,
            i18n::line::term_overview_gauge,
            &line.current.gauge,
            |gauge, cont| {
                for gauge in gauge.iter() {
                    write!(cont, "{}", gauge);
                }
            }
        );

        // tracks
        opt_overview_part(
            site, line, table,
            i18n::line::term_overview_tracks,
            &line.current.tracks,
            |tracks, cont| {
                write!(cont, "{}", tracks);
            }
        );

        // electrified
        opt_overview_part(
            site, line, table,
            i18n::line::term_overview_electrified,
            &line.current.electrified,
            |electrified, cont| {
                if let Some(system) = electrified {
                    cont.text(system)
                }
                else {
                    cont.text(i18n::line::term_not_electrified)
                }
            }
        );

        // passenger
        opt_overview_part(
            site, line, table,
            i18n::line::term_overview_passenger,
            &line.current.passenger,
            |passenger, cont| cont.text(*passenger),
        );

        // goods
        opt_overview_part(
            site, line, table,
            i18n::line::term_overview_goods,
            &line.current.goods,
            |goods, cont| cont.text(*goods)
        );

        // owner
        opt_overview_part(
            site, line, table,
            i18n::line::term_overview_owner,
            &line.current.owner,
            |operator, cont| {
                for operator in operator {
                    organization::link(site, operator.into_value(), cont);
                    cont.br();
                }
            }
        );

        // operator
        opt_overview_part(
            site, line, table,
            i18n::line::term_overview_operator,
            &line.current.operator,
            |operator, cont| {
                for operator in operator {
                    organization::link(site, operator.into_value(), cont);
                    cont.br();
                }
            }
        );

        // de_vzg
        opt_overview_opt_part(
            site, line, table,
            "VzG",
            &line.current.de_vzg,
            |vzg, cont| cont.text(vzg.as_value().as_str())
        );
    });
}

fn overview_course(site: &Site, line: &Line, table: &mut PropertyTable) {
    table.property(
        |cont| cont.text(i18n::line::term_overview_course),
        |cont| {
            let jur = line.jurisdiction();
            cont.strong().content(|cont| {
                point::link_in_jurisdiction(
                    cont, line.points[0].follow(site.library()),
                    jur, site
                )
            });
            for link in &line.points[1..line.points.len() - 1] {
                let point = link.follow(site.library());
                match point.junction {
                    Some(val) => {
                        if !val.into_value() {
                            continue
                        }
                    }
                    None => {
                        if !site.catalogue().point_connections
                                .is_junction(link.into_value())
                        {
                            continue
                        }
                    }
                }
                cont.text(i18n::line::term_course_separator);
                point::link_in_jurisdiction(cont, point, jur, site);
            }
            cont.strong().content(|cont| {
                cont.text(i18n::line::term_course_separator);
                point::link_in_jurisdiction(
                    cont, line.points.last().unwrap().follow(site.library()),
                    jur, site
                )
            });
        }
    );
}

fn overview_part<T, F: Fn(&T, &mut Content)> (
    site: &Site, line: &Line,
    table: &mut PropertyTable,
    name: impl RenderText,
    value: &CurrentValue<T>,
    render: F,
) {
    table.property(
        |cont| cont.text(name),
        |cont| {
            if value.is_empty() {
                cont.i().text(i18n::line::term_not_available)
            }
            else if value.len() == 1 {
                render(&value[0].1, cont)
            }
            else {
                cont.table().class("line-overview-part").content(|cont| {
                    for &(ref section, ref value) in value.iter() {
                        cont.tr().content(|cont| {
                            cont.th().content(|cont| {
                                line_segment(
                                    site, line, section, cont
                                )
                            });
                            cont.td().content(|cont| render(value, cont));
                        });
                    }
                })
            }
        }
    );
}

fn opt_overview_part<T, F: Fn(&T, &mut Content)> (
    site: &Site, line: &Line,
    table: &mut PropertyTable,
    name: impl RenderText,
    value: &CurrentValue<T>,
    render: F,
) {
    if !value.is_empty() {
        overview_part(site, line, table, name, value, render)
    }
}

fn overview_opt_part<T, F: Fn(&T, &mut Content)> (
    site: &Site, line: &Line,
    table: &mut PropertyTable,
    name: impl RenderText,
    value: &CurrentValue<Option<T>>,
    render: F,
) {
    table.property(
        |cont| cont.text(name),
        |cont| {
            if value.is_empty() {
                cont.i().text(i18n::line::term_not_available)
            }
            else if value.len() == 1 {
                match value[0].1 {
                    Some(ref value) => render(value, cont),
                    None => cont.text(i18n::line::term_no_value)
                }
            }
            else {
                cont.table().class("line-overview-part").content(|cont| {
                    for &(ref section, ref value) in value.iter() {
                        cont.tr().content(|cont| {
                            cont.th().content(|cont| {
                                line_segment(
                                    site, line, section, cont
                                )
                            });
                            cont.td().content(|cont| {
                                match value.as_ref() {
                                    Some(value) => render(value, cont),
                                    None =>
                                        cont.text(i18n::line::term_no_value),
                                }
                            })
                        });
                    }
                })
            }
        }
    );
}

fn opt_overview_opt_part<T, F: Fn(&T, &mut Content)> (
    site: &Site, line: &Line,
    table: &mut PropertyTable,
    name: impl RenderText,
    value: &CurrentValue<Option<T>>,
    render: F,
) {
    if !value.is_empty() {
        overview_opt_part(site, line, table, name, value, render)
    }
}


//------------ route_section -------------------------------------------------

fn route_section(site: &Site, line: &Line, cont: &mut Content) {
    cont.table().class("line-route-table").content(|cont| {
        cont.thead().content(|cont| cont.tr().content(|cont| {
            cont.th().class("line-route-location").text(
                i18n::line::term_route_location
            );
            cont.th().text(
                i18n::line::term_route_category
            );
            cont.th().text(
                i18n::line::term_route_name
            );
            cont.th().text(
                i18n::line::term_route_connections
            );
        }));
        cont.tbody().content(|cont| {
            for point in line.points.iter() {
                route_point(site, line, point.into_value(), cont);
            }
        })
    })
}

fn route_point(
    site: &Site, line: &Line, link: PointLink, cont: &mut Content
) {
    let point = link.follow(site.library());
    let mut tr = cont.tr();
    if !point.is_open() {
        tr = tr.class("here-closed");
    }
    tr.content(|cont| {
        cont.td().class("line-route-location").content(|cont| {
            // XXX Currently, all locations use a comma as the decimal
            //     separator. This will have to become more advanced if we
            //     go to other countries.
            if let Some((loc, _changed)) = point.location(line.link()) {
                if let Some(loc) = loc {
                    if let Some(idx) = loc.find(',') {
                        let (left, right) = loc.split_at(idx);
                        cont.span().class("here-left").text(left);
                        cont.span().class("here-right").text(right);
                    }
                    else {
                        cont.text(loc)
                    }
                }
                else {
                    cont.text("\u{2014}");
                }
            }
        });
        cont.td().class("line-route-category").content(|cont| {
            if let Some((cat, _changed)) = point.category() {
                for item in cat {
                    cont.text(item.code());
                    cont.text(" ");
                }
            }
        });
        cont.td().class("line-route-name").content(|cont| {
            cont.a().href(site.link_point(point)).content(|cont| {
                cont.text(point.name_in_jurisdiction(line.jurisdiction()));
            });
        });
        cont.td().class("line-route-connections").content(|cont| {
            let lines = site.catalogue().point_connections.get_lines(link);
            if lines.len() > 2 {
                cont.ul().content(|cont| {
                    for &item in lines {
                        if item != line.link() {
                            let other = item.follow(site.library());
                            cont.li().content(|cont| {
                                cont.a().href(site.link_line(other))
                                .text(other.name(site.lang().into()));
                            })
                        }
                    }
                })
            }
        });
    })
}


//------------ events_section ------------------------------------------------

fn events_section(site: &Site, line: &Line, cont: &mut Content) {
    let mut sources = SourceList::new("event-source");
    cont.table().class("line-events-table").content(|cont| {
        for item in &line.events {
            event(site, line, item, &mut sources, cont);
        }
    });
    sources.render(cont, site);
}

fn event(
    site: &Site, line: &Line, event: &Event,
    sources: &mut SourceList,
    cont: &mut Content
) {
    if !event.is_legal() && !event.has_properties() {
        return
    }
    cont.tr().content(|cont| {
        cont.td().class("line-events-date").text(&event.date);
        cont.td().class("line-events-facts").content(|cont| {
            cont.dl().class("line-event").content(|cont| {
                for section in &event.sections {
                    cont.dt().content(|cont| {
                        line_segment(site, line, section, cont);
                    });
                }
                cont.dd().content(|cont| {
                    if event.is_legal() {
                        event_content_legal(cont, event, site);
                    }
                    else if event.status == Some(Status::Open) {
                        event_content_open(cont, event, site);
                    }
                    else if event.date.is_empty() {
                        event_content_no_date(cont, event, site);
                    }
                    else {
                        event_content_other(cont, event, site);
                    }

                    // alternative
                    for alt in &event.alternative {
                        cont.p().content(|cont| {
                            common::event_alternative(cont, alt, site, sources);
                        })
                    }

                    // basis
                    if !event.basis.is_empty() {
                        cont.p().content(|cont| {
                            cont.ul().content(|cont| {
                                for basis in &event.basis {
                                    cont.li().content(|cont| {
                                        common::event_basis(
                                             cont, basis, site, sources
                                        );
                                    });
                                }
                            })
                        });
                    }
                });
            });
        });
        cont.td().class("line-events-source").content(|cont| {
            for link in event.document.iter().chain(event.source.iter()) {
                let num = sources.add(link.into_value());
                cont.a()
                    .href(|text: &mut Text| write!(text, "#source-{}", num))
                .text(|text: &mut Text| write!(text, "[{}]", num));
            }
        })
    })
}

fn event_content_legal(
    cont: &mut Content,
    event: &Event,
    site: &Site,
) {
    if let Some(ref concession) = event.concession {
        cont.p().content(|cont| {
            i18n::line::event_concession(cont, concession, &event.date, site);
        })
    }
    if let Some(ref agreement) = event.agreement {
        cont.p().content(|cont| {
            common::event_agreement(cont, agreement, &event.date, site)
        })
    }
    if event.has_properties() {
        cont.ul().content(|cont| {
            records_content_properties(cont, event, &event.date, site)
        })
    }
}

fn event_content_open(
    cont: &mut Content,
    event: &Event,
    site: &Site,
) {
    cont.p().content(|cont| cont.text(i18n::line::term_event_opened));
    cont.ul().content(|cont| {
        records_content_properties_no_status(
            cont, event, &event.date, site
        );
    })
}

fn event_content_no_date(
    cont: &mut Content,
    event: &Event,
    site: &Site,
) {
    cont.ul().content(|cont| {
        records_content_properties_no_status(
            cont, event, &event.date, site
        );
    })
}

fn event_content_other(
    cont: &mut Content,
    event: &Event,
    site: &Site,
) {
    if let Some(status) = event.status {
        cont.p().content(|cont| {
            i18n::line::event_status(cont, status)
        })
    }
    if let Some(ref name) = event.name {
        cont.p().content(|cont| i18n::line::event_name(cont, name));
    }
    if let Some(ref category) = event.category {
        cont.p().content(|cont| {
            i18n::line::event_category(cont, category);
        });
    }
    if let Some(ref gauge) = event.gauge {
        cont.p().content(|cont| i18n::line::event_gauge(cont, gauge));
    }
    if let Some(rails) = event.rails {
        cont.p().content(|cont| {
            i18n::line::event_rails(cont, rails.into_value());
        });
    }
    if let Some(tracks) = event.tracks {
        cont.p().content(|cont| {
            i18n::line::event_tracks(cont, tracks.into_value());
        });
    }
    if let Some(ref electrified) = event.electrified {
        cont.p().content(|cont| {
            match electrified.as_ref() {
                Some(electrified) => {
                    i18n::line::event_electrified(cont, electrified);
                }
                None => {
                    cont.text(i18n::line::term_event_unelectrified);
                }
            }
        })
    }

    if let Some(goods) = event.goods {
        cont.p().content(|cont| i18n::line::event_goods(cont, goods))
    }
    if let Some(pax) = event.passenger {
        cont.p().content(|cont| i18n::line::event_passenger(cont, pax))
    }

    if let Some(ref owner) = event.owner {
        cont.p().content(|cont| {
            i18n::line::event_owner(cont, owner, &event.date, site);
        })
    }
    if let Some(ref constructor) = event.constructor {
        cont.p().content(|cont| {
            i18n::line::event_constructor(
                cont, constructor, &event.date, site
            );
        })
    }
    if let Some(ref operator) = event.operator {
        cont.p().content(|cont| {
            i18n::line::event_operator(cont, operator, &event.date, site);
        })
    }

    if let Some(jur) = event.jurisdiction {
        cont.p().content(|cont| {
            i18n::line::event_jurisdiction(cont, jur.into_value())
        });
    }

    if let Some(ref code) = event.de_vzg {
        cont.p().content(|cont| i18n::line::event_de_vzg(cont, code));
    }

}


//------------ records_section -----------------------------------------------

fn records_section(site: &Site, line: &Line, cont: &mut Content) {
    for (source, records) in line.records.documents() {
        cont.div().class("core-subpart").content(|cont| {
            record(site, line, source, records, cont);
        });
    }
}

fn record(
    site: &Site, line: &Line,
    source_link: SourceLink, records: &List<Record>,
    cont: &mut Content
) {
    let source = source_link.follow(site.library());
    cont.h2().content(|cont| {
        cont.a().href(site.link_source(source)).content(|cont| {
            source::short_title(cont, source, site)
        });
    });
    cont.dl().class("line-event").content(|cont| {
        for record in records {
            for section in &record.sections {
                cont.dt().content(|cont| {
                    line_segment(site, line, section, cont);
                });
            }
            cont.dd().content(|cont| cont.ul().content(|cont| {
                records_content_properties(cont, record, &source.date, site);
            }));
        }
    });
}

fn records_content_properties(
    cont: &mut Content,
    record: &Properties,
    date: &EventDate,
    site: &Site,
) {
    if let Some(status) = record.status {
        cont.li().content(|cont| {
            i18n::line::record_status(cont, status)
        })
    }
    records_content_properties_no_status(
        cont, record, date, site
    );
}

fn records_content_properties_no_status(
    cont: &mut Content,
    event: &Properties,
    date: &EventDate,
    site: &Site,
) {
    if let Some(ref name) = event.name {
        cont.li().content(|cont| i18n::line::record_name(cont, name));
    }
    if let Some(ref category) = event.category {
        cont.li().content(|cont| i18n::line::record_category(cont, category));
    }
    if let Some(ref gauge) = event.gauge {
        cont.li().content(|cont| i18n::line::record_gauge(cont, gauge));
    }
    if let Some(rails) = event.rails {
        cont.li().content(|cont| {
            i18n::line::record_rails(cont, rails.into_value());
        });
    }
    if let Some(tracks) = event.tracks {
        cont.li().content(|cont| {
            i18n::line::record_tracks(cont, tracks.into_value());
        });
    }
    if let Some(ref electrified) = event.electrified {
        cont.li().content(|cont| {
            match electrified.as_ref() {
                Some(el) => i18n::line::record_electrified(cont, el),
                None => cont.text(i18n::line::record_not_electrified),
            }
        })
    }

    if let Some(goods) = event.goods {
        cont.li().content(|cont| i18n::line::record_goods(cont, goods))
    }
    if let Some(pax) = event.passenger {
        cont.li().content(|cont| i18n::line::record_passenger(cont, pax))
    }

    if let Some(ref owner) = event.owner {
        cont.li().content(|cont| {
            i18n::line::record_owner(cont, owner, date, site);
        })
    }
    if let Some(ref constr) = event.constructor {
        cont.li().content(|cont| {
            i18n::line::record_constructor(cont, constr, date, site);
        })
    }
    if let Some(ref operator) = event.operator {
        cont.li().content(|cont| {
            i18n::line::record_operator(cont, operator, date, site);
        })
    }

    if let Some(jur) = event.jurisdiction {
        cont.li().content(|cont| {
            i18n::line::record_jurisdiction(cont, jur.into_value())
        });
    }

    if let Some(ref code) = event.de_vzg {
        cont.li().content(|cont| i18n::line::record_de_vzg(cont, code));
    }
}



//------------ Components ----------------------------------------------------

fn line_segment(
    site: &Site, line: &Line,
    section: &Section,
    cont: &mut Content
) {
    let jurisdiction = line.jurisdiction();
    let start = section.start.unwrap_or_else(|| {
        *line.points.first().unwrap()
    }).follow(site.library());
    let end = section.end.unwrap_or_else(|| {
        *line.points.last().unwrap()
    }).follow(site.library());

    cont.a().href(site.link_point(start)).text(
        start.name_in_jurisdiction(jurisdiction)
    );
    cont.text("\u{202f}\u{2013}\u{202f}");
    cont.a().href(site.link_point(end)).text(
        end.name_in_jurisdiction(jurisdiction)
    );
}


//------------ LineTitle -----------------------------------------------------

pub fn title(cont: &mut Content, line: &Line, site: &Site) {
    cont.text(LineTitle { site, line })
}

struct LineTitle<'a> {
    site: &'a Site,
    line: &'a Line,
}

impl<'a> RenderText for LineTitle<'a> {
    fn render(self, target: &mut Text) {
        let jurisdiction = self.line.jurisdiction();
        let first = self.line.points.first().unwrap().follow(
            self.site.library()
        );
        let last = self.line.points.last().unwrap().follow(
            self.site.library()
        );
        write!(
            target, "{} – {}",
            first.name_in_jurisdiction(jurisdiction),
            last.name_in_jurisdiction(jurisdiction)
        )
    }
}


//------------ Line Links ----------------------------------------------------

pub fn link(cont: &mut Content, line: &Line, site: &Site) {
    cont.a().href(site.link_line(line)).content(|cont| {
        cont.text(line.code())
    })
}

