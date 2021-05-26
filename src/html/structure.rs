use raildata::document::structure::Structure;
use raildata::types::LocalText;
use crate::site::Site;
use crate::site::structure::Part;
use super::{i18n, skeleton};
use super::components::{property_view, stub_badge};
use super::target::{Content, Target, empty};

//------------ details -------------------------------------------------------

pub fn details(site: &Site, structure: &Structure, part: Part) -> Target {
    skeleton::multipart_sheet(
        site,
        structure.name(site.lang().into()),
        empty,
        skeleton::Nav::Other,
        |cont| {
            cont.h1().text(structure.name(site.lang().into()))
        },
        |nav| {
            nav.item(
                site.link_structure_part(structure, Part::Overview),
                matches!(part, Part::Overview),
                |cont| cont.text(i18n::structure::term_part_overview)
            );
        },
        |cont| {
            match part {
                Part::Overview => overview_part(cont, structure, site),
            }
        },
        empty,
    )
}


//------------ overview_part -------------------------------------------------

fn overview_part(cont: &mut Content, structure: &Structure, _site: &Site) {
    stub_badge(cont);

    property_view(cont, |view| {
        // name
        view.property(
            |name| name.text(i18n::structure::term_overview_name),
            |values| {
                let mut any = false;
                for event in &structure.events {
                    if let Some(name) = event.name.as_ref() {
                        values.dated(
                            |date| date.text(&event.date),
                            |value| local_name(value, name),
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

        // length
        view.property(
            |name| name.text(i18n::structure::term_overview_length),
            |values| {
                let mut any = false;
                for event in &structure.events {
                    if let Some(length) = event.length {
                        values.dated(
                            |date| date.text(&event.date),
                            |value| {
                                value.text(
                                    format_args!("{:.1}\u{202f}m", length)
                                )
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
    })
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

