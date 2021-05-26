//! Translations related to point documents.

use raildata::document::LineLink;
use raildata::document::point::{
    Category, CodeType, DeRang, Point, ServiceSet, Status
};
use crate::html::{line, point};
use crate::html::target::Content;
use crate::i18n::Lang::*;
use crate::site::Site;
use super::utils::{and_join, join};


str_terms! {
    term_part_overview {
        En => "Overview",
        De => "Übersicht",
    }
    term_part_events {
        En => "Events",
        De => "Ereignisse",
    }
    term_part_records {
        En => "Records",
        De => "Belege",
    }

    term_overview_status {
        En => "Status",
        De => "Status",
    }
    term_overview_name {
        En => "Name",
        De => "Name",
    }
    term_overview_category {
        En => "Category",
        De => "Kategorie",
    }
    term_overview_location {
        En => "Location",
        De => "Lage",
    }
    term_overview_subtype {
        En => "Type",
        De => "Art",
    }

    term_overview_designation_footnote {
        En => "* not an offical name",
        De => "* kein offizieller Name",
    }

    term_event_location {
        En => "Location: ",
        De => "Lage: ",
    }
}

lang_enum! {
    raildata::document::point::CodeType {
        En => {
            Plc => "Primary location code",
            DeDs100 => "DS 100 code",
            DeDstnr => "Dst no",
            DeLknr => "Punch card no.",
            DeVbl => "VBL code",
            DkRef => "Reference code",
            NoFs => "Radio code",
            NoNjk => "NJK code",
            NoNsb => "NSB code",
        }
        De => {
            Plc => "Primary Location Code",
            DeDs100 => "DS 100",
            DeDstnr => "Dst-Nr",
            DeLknr => "Lochkarten-Nr",
            DeVbl => "Abk. VBL",
            DkRef => "Referenzcode",
            NoFs => "Fernschreibkürzel",
            NoNjk => "Abk. NJK",
            NoNsb => "Abk. NSB",
        }
    }

    raildata::document::point::Staff {
        En => {
            Full => "full staff",
            Agent => "staffed by an agent",
            None => "unstaffed",
        }
        De => {
            Full => "besetzt",
            Agent => "mit einem Agenten besetzt",
            None => "unbesetzt",
        }
    }

    raildata::document::point::Status {
        En => {
            Open => "open",
            Suspended => "suspended",
            Reopened => "reopened",
            Closed => "closed",
        }
        De => {
            Open => "in Betrieb",
            Suspended => "zeitweilig außer Betrieb",
            Reopened => "wieder in Betrieb",
            Closed => "aufgelassen",
        }
    }

    raildata::document::point::Subtype {
        En => {
            Border => "border",
            Break => "mileage discontinuity",
            Post => "post",
            Reference => "reference point",
        }
        De => {
            Border => "Grenze",
            Break => "Fehlstelle",
            Post => "Betriebsstelle",
            Reference => "Referenzpunkt",
        }
    }
}

lang_enum_fn! {
    raildata::document::point::ServiceRate as event_passenger {
        En => {
            None => "no passenger service",
            Limited => "limited passenger service",
            Full => "full passenger service",
        }
        De => {
            None => "kein Personenverkehr",
            Limited => "beschränkter Personenverkehr",
            Full => "voller Personenverkehr",
        }
    }

    raildata::document::point::ServiceRate as event_luggage {
        En => {
            None => "no luggage service",
            Limited => "limited luggage service",
            Full => "full luggage service",
        }
        De => {
            None => "kein Gepäckverkehr",
            Limited => "beschränkter Gepäckverkehr",
            Full => "voller Gepäckverkehr",
        }
    }

    raildata::document::point::ServiceRate as event_express {
        En => {
            None => "no express goods service",
            Limited => "limited express goods service",
            Full => "full express goods service",
        }
        De => {
            None => "kein Expressgutverkehr",
            Limited => "beschränkter Expressgutverkehr",
            Full => "voller Expressgutverkehr",
        }
    }

    raildata::document::point::ServiceRate as event_goods {
        En => {
            None => "no goods service",
            Limited => "limited goods service",
            Full => "full goods service",
        }
        De => {
            None => "kein Güterverkehr",
            Limited => "beschränkter Güterverkehr",
            Full => "voller Güterverkehr",
        }
    }
}


//------------ Event Details -------------------------------------------------

pub fn event_split_from(
    cont: &mut Content,
    point: &Point,
    site: &Site
) {
    match cont.lang() {
        En => {
            cont.text("Split from ");
            point::link(cont, point, site);
            cont.text(".");
        }
        De => {
            cont.text("Aus ");
            point::link(cont, point, site);
            cont.text(" ausgegliedert.");
        }
    }
}

pub fn event_merged(
    cont: &mut Content,
    point: &Point,
    site: &Site
) {
    match cont.lang() {
        En => {
            cont.text("Merged into ");
            point::link(cont, point, site);
            cont.text(".");
        }
        De => {
            cont.text("In ");
            point::link(cont, point, site);
            cont.text(" eingegliedert.");
        }
    }
}

pub fn event_status(cont: &mut Content, status: Status) {
    cont.text(match cont.lang() {
        En => match status {
            Status::Open => "Opened.",
            Status::Suspended => "Suspended.",
            Status::Reopened => "Re-opened.",
            Status::Closed => "Closed.",
        }
        De => match status {
            Status::Open => "Eröffnet.",
            Status::Suspended => "Zeitweilig geschlossen.",
            Status::Reopened => "Wiedereröffnet.",
            Status::Closed => "Aufgelassen.",
        }
    })
}

pub fn event_de_name16(cont: &mut Content, name: &str) {
    match cont.lang() {
        En => {
            cont.text(format_args!(
                "Short name (16 characters): {}.", name
            ))
        }
        De => {
            cont.text(format_args!(
                "Kurzname (16 Zeichen): {}.", name
            ))
        }
    }
}

pub fn event_category(
    cont: &mut Content, cat: impl Iterator<Item = Category>
) {
    match cont.lang() {
        En => {
            cont.text("Category:")
        }
        De => {
            cont.text("Art:")
        }
    }
    for item in cat {
        cont.text(" ");
        cont.text(item.code());
    }
    cont.text(".");
}

pub fn event_de_rang(cont: &mut Content, rang: DeRang) {
    match cont.lang() {
        En => {
            cont.text(format_args!(
                "Station rank: {}.", rang
            ));
        }
        De => {
            cont.text(format_args!(
                "Bahnhofsklasse: {}.", rang
            ));
        }
    }
}

pub fn event_no_superior(cont: &mut Content) {
    match cont.lang() {
        En => cont.text("Independent point."),
        De => cont.text("Selbstständige Betriebsstelle."),
    }
}

pub fn event_one_superior(cont: &mut Content, point: &Point, site: &Site) {
    match cont.lang() {
        En => {
            cont.text("Superior point: ");
            point::link(cont, point, site);
            cont.text(".")
        }
        De => {
            cont.text("Übergeordnete Betriebsstelle: ");
            point::link(cont, point, site);
            cont.text(".")
        }
    }
}

pub fn event_many_superiors<'a, Iter: Iterator<Item = &'a Point> + 'a>(
    cont: &mut Content, points: Iter, site: &Site
) {
    match cont.lang() {
        En => {
            cont.text("Superior points: ");
            and_join(cont, points, |cont, point| {
                point::link(cont, point, site)
            });
            cont.text(".")
        }
        De => {
            cont.text("Übergeordnete Betriebsstellen: ");
            and_join(cont, points, |cont, point| {
                point::link(cont, point, site)
            });
            cont.text(".")
        }
    }
}

pub fn event_code<'a>(
    cont: &mut Content,
    code_type: CodeType,
    codes: impl Iterator<Item = &'a str> + 'a
) {
    cont.text(format_args!("{}: ", code_type));
    join(cont, codes, |cont, code| {
        cont.text(code)
    });
    cont.text(".")
}

pub fn event_location<'a>(
    cont: &mut Content,
    iter: impl Iterator<Item = (LineLink, Option<&'a str>)>,
    site: &Site,
) {
    cont.text(term_event_location);
    cont.ul().class("point-events-location").content(|cont| {
        for (line, location) in iter {
            cont.li().content(|cont| {
                line::link(cont, line.follow(site.library()), site);
                cont.text(": ");
                cont.text(location.unwrap_or("\u{2014}"));
            })
        }
    })
}

pub fn event_service(
    cont: &mut Content,
    service: ServiceSet
) {
    cont.ul().class("point-events-service").content(|cont| {
        if let Some(rate) = service.passenger {
            cont.li().content(|cont| event_passenger(cont, rate));
        }
        if let Some(rate) = service.luggage {
            cont.li().content(|cont| event_luggage(cont, rate));
        }
        if let Some(rate) = service.express {
            cont.li().content(|cont| event_express(cont, rate));
        }
        if let Some(rate) = service.goods {
            cont.li().content(|cont| event_goods(cont, rate));
        }
    })
}
