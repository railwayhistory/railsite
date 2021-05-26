//! Translations for line document pages.

use raildata::document::OrganizationLink;
use raildata::document::line::{
    Category, Concession, DeVzg, Electrified, Gauge, Goods, Passenger, Status
};
use raildata::types::{CountryCode, EventDate, List, LocalText, Marked, Set};
use crate::i18n::Lang::*;
use crate::site::Site;
use super::super::organization;
use super::super::target::{Content, RenderText, Text};
use super::utils::{and_join, join};


str_terms! {
    term_section_overview {
        En => "Current",
        De => "Zustand",
    }
    term_section_route {
        En => "Route",
        De => "Betriebsstellen",
    }
    term_section_events {
        En => "Events",
        De => "Ereignisse",
    }
    term_section_records {
        En => "Records",
        De => "Belege",
    }

    term_overview_course {
        En => "Course",
        De => "Verlauf",
    }
    term_overview_category {
        En => "Category",
        De => "Kategorie",
    }
    term_overview_electrified {
        En => "Electrification",
        De => "Elektrischer Betrieb",
    }
    term_overview_gauge {
        En => "Gauge",
        De => "Spurweite",
    }
    term_overview_goods {
        En => "Goods service",
        De => "Güterverkehr",
    }
    term_overview_operator {
        En => "Operator",
        De => "Betriebsführer",
    }
    term_overview_owner {
        En => "Owner",
        De => "Eigentümer",
    }
    term_overview_passenger {
        En => "Passenger service",
        De => "Personenverkehr",
    }
    term_overview_status {
        En => "Status",
        De => "Status",
    }
    term_overview_tracks {
        En => "Tracks",
        De => "Gleise",
    }

    term_route_category {
        En => "Category",
        De => "Kategorie",
    }
    term_route_connections{
        En => "Connections",
        De => "Verbindungen",
    }
    term_route_location {
        En => "Location",
        De => "Lage",
    }
    term_route_name{
        En => "Name",
        De => "Bezeichnung",
    }

    term_not_electrified {
        En => "not electrified",
        De => "kein elektrischer Betrieb",
    }
    term_not_available {
        En => "N/A",
        De => "unbekannt",
    }
    term_no_value {
        En => "—",
        De => "—",
    }
    term_event_opened {
        En => "Line opening.",
        De => "Streckeneröffnung.",
    }
    term_event_unelectrified {
        En => "Electric operation discontinued.",
        De => "Einstellung des elektrischen Betriebs.",
    }

    term_course_separator {
        En => "\u{202f}\u{2013}\u{2009}",
        De => "\u{202f}\u{2013}\u{2009}",
    }
}

lang_fn! {
    line_code(code: impl std::fmt::Display) {
        En => { "Line {}", code }
        De => { "Strecke {}", code }
    }
}

lang_enum! {
    raildata::document::line::Category {
        En => {
            DeHauptbahn => "primary line",
            DeNebenbahn => "secondary line",
            DeKleinbahn => "light railway",
            DeAnschl => "industrial railway",
            DeBfgleis => "station track",
            DeStrab => "tram",
        }
        De => {
            DeHauptbahn => "Hauptbahn",
            DeNebenbahn => "Nebenbahn",
            DeKleinbahn => "Kleinbahn",
            DeAnschl => "Anschlußgleis",
            DeBfgleis => "Bahnhofsgleis",
            DeStrab => "Straßenbahn",
        }
    }

    raildata::document::line::ConcessionRight {
        En => {
            Construction => "construction",
            Operation => "operation",
            Expropriation => "expropriation",
        }
        De => {
            Construction => "Bau",
            Operation => "Betrieb",
            Expropriation => "Enteignung",
        }
    }

    raildata::document::line::Goods {
        En => {
            None => "none",
            Limited=> "limited",
            Full => "full",
        }
        De => {
            None => "keiner",
            Limited => "beschränkt",
            Full => "voll",
        }
    }

    raildata::document::line::Passenger {
        En => {
            None => "none",
            Limited => "limited",
            Historic => "historic",
            Seasonal => "seasonal",
            Tourist => "touristic",
            Full => "full",
        }
        De => {
            None => "keiner",
            Limited => "beschränkt",
            Historic => "Museumsverkehr",
            Seasonal =>  "saisonal",
            Tourist => "Ausflugsverkehr",
            Full => "voll",
        }
    }

    raildata::document::line::Status {
        En => {
            Planned => "planned",
            Construction => "under construction",
            Open => "open",
            Suspended => "suspended",
            Reopened => "open",
            Closed => "permanently closed",
            Removed => "removed",
            Released => "released",
        }
        De => {
            Planned => "geplant",
            Construction => "im Bau",
            Open => "in Betrieb",
            Suspended => "gesperrt",
            Reopened => "in Betrieb",
            Closed => "stillgelegt",
            Removed => "abgebaut",
            Released => "entwidmet",
        }
    }
}


//------------ Concession ----------------------------------------------------

pub fn event_concession(
    cont: &mut Content,
    concession: &Concession,
    date: &EventDate,
    site: &Site, 
) {
    match cont.lang() {
        En => {
            cont.text("Concession ");
            if !concession.rights.is_empty() {
                cont.text("for ");
                and_join(cont, concession.rights.iter(), |cont, right| {
                    cont.text(right.into_value())
                });
            }
            cont.text(" granted");
            if !concession.by.is_empty() {
                cont.text(" by ");
                and_join(cont, concession.by.iter(), |cont, org| {
                    organization::link_at(cont, org.into_value(), date, site);
                });
            }
            if !concession.to.is_empty() {
                cont.text(" to ");
                and_join(cont, concession.to.iter(), |cont, org| {
                    organization::link_at(cont, org.into_value(), date, site);
                });
            }
            if let Some(ref until) = concession.until {
                cont.text(" until ");
                cont.text(until.as_value());
            }
            cont.text(".");
        }
        De => {
            cont.text("Genehmigung ");
            if !concession.rights.is_empty() {
                cont.text("zu ");
                and_join(cont, concession.rights.iter(), |cont, right| {
                    cont.text(right.into_value())
                });
            }
            cont.text(" erteilt");
            if !concession.by.is_empty() {
                cont.text(" durch ");
                and_join(cont, concession.by.iter(), |cont, org| {
                    organization::link_at(cont, org.into_value(), date, site);
                });
            }
            if !concession.to.is_empty() {
                cont.text(" an ");
                and_join(cont, concession.to.iter(), |cont, org| {
                    organization::link_at(cont, org.into_value(), date, site);
                });
            }
            if let Some(ref until) = concession.until {
                cont.text(", gültig bis ");
                cont.text(until.as_value());
            }
            cont.text(".");
        }
    }
}


//------------ Electrified ---------------------------------------------------

impl<'a> RenderText for &'a Electrified {
    fn render(self, target: &mut Text) {
        let lang = target.lang();

        target.push_str(match self.as_value().as_str() {
            "de" => match lang {
                En => "OCL 15\u{202f}kV 16.7\u{202f}Hz",
                De => "Oberleitung 15\u{202f}kV 16,7\u{202f}Hz",
            }
            other => other,
        })
    }
}


//------------ Gauge ---------------------------------------------------------

impl<'a> RenderText for &'a Gauge {
    fn render(self, target: &mut Text) {
        target.write_fmt(format_args!("{}", self));
    }
}


//------------ event properties ----------------------------------------------
//
// This are written as if they had changed at the event.

pub fn event_category(cont: &mut Content, category: &Set<Category>) {
    match cont.lang() {
        En => {
            cont.text("Category changed to ");
            join(cont, category.iter(), |cont, cat| cont.text(*cat))
        }
        De => {
            cont.text("Reklassifiziert als ");
            join(cont, category.iter(), |cont, cat| cont.text(*cat))
        }
    }
}

pub fn event_constructor(
    cont: &mut Content,
    constructor: &List<Marked<OrganizationLink>>,
    date: &EventDate,
    site: &Site
) {
    match cont.lang() {
        En => {
            cont.text("Construction taken over by ");
            and_join(cont, constructor.iter(), |cont, org| {
                organization::link_at(
                    cont, org.into_value(), date, site
                );
            });
            cont.text(".");
        }
        De => {
            cont.text("Erbauung übernommen durch ");
            and_join(cont, constructor.iter(), |cont, org| {
                organization::link_at(
                    cont, org.into_value(), date, site
                );
            });
            cont.text(".");
        }
    }
}

pub fn event_electrified(
    cont: &mut Content,
    electrified: &Set<Electrified>,
) {
    match cont.lang() {
        En => {
            cont.text("Begin of electric operation with ");
            and_join(cont, electrified.iter(), |c, el| c.text(el));
            cont.text(".");
        }
        De => {
            cont.text("Elektrischer Betrieb aufgenommen mit ");
            and_join(cont, electrified.iter(), |c, el| c.text(el));
            cont.text(".");
        }
    }
}

pub fn event_gauge(cont: &mut Content, gauge: &Set<Gauge>) {
    match cont.lang() {
        En => {
            cont.text("Track gauge changed to ");
            and_join(cont, gauge.iter(), |cont, gauge| cont.text(gauge));
        }
        De => {
            cont.text("Umgespurt auf ");
            and_join(cont, gauge.iter(), |cont, gauge| cont.text(gauge));
        }
    }
}

pub fn event_goods(cont: &mut Content, goods: Goods) {
    match cont.lang() {
        En => {
            cont.text(match goods {
                Goods::None => "Goods service ceased.",
                Goods::Limited => "Limited goods service only.",
                Goods::Full => "Full goods service started.",
            })
        }
        De => {
            cont.text(match goods {
                Goods::None => "Güterverkehr eingestellt.",
                Goods::Limited => "Beschränkter Güterverkehr.",
                Goods::Full => "Voller Güterverkehr aufgenommen.",
            })
        }
    }
}

pub fn event_jurisdiction(cont: &mut Content, code: CountryCode) {
    match cont.lang() {
        En => {
            cont.text("Under jurisdiction of ");
            cont.text(code);
            cont.text(".");
        }
        De => {
            cont.text("Unter Jurisdiktion von ");
            cont.text(code);
            cont.text(".");
        }
    }
}

pub fn event_name(cont: &mut Content, name: &LocalText) {
    let name = name.for_language(cont.lang().into()).unwrap_or_else(|| {
        name.first()
    });
    match cont.lang() {
        En => {
            cont.text("Renamed into ");
            cont.text(name);
            cont.text(".");
        }
        De => {
            cont.text("Umbenannt in ");
            cont.text(name);
            cont.text(".");
        }
    }
}

pub fn event_operator(
    cont: &mut Content,
    operator: &List<Marked<OrganizationLink>>,
    date: &EventDate,
    site: &Site
) {
    match cont.lang() {
        En => {
            cont.text("Operation taken over by ");
            and_join(cont, operator.iter(), |cont, org| {
                organization::link_at(
                    cont, org.into_value(), date, site
                );
            });
            cont.text(".");
        }
        De => {
            cont.text("Betriebsführung übernommen durch ");
            and_join(cont, operator.iter(), |cont, org| {
                organization::link_at(
                    cont, org.into_value(), date, site
                );
            });
            cont.text(".");
        }
    }
}

pub fn event_owner(
    cont: &mut Content,
    owner: &List<Marked<OrganizationLink>>,
    date: &EventDate,
    site: &Site
) {
    match cont.lang() {
        En => {
            cont.text("Ownership transferred to ");
            and_join(cont, owner.iter(), |cont, org| {
                organization::link_at(
                    cont, org.into_value(), date, site
                );
            });
            cont.text(".");
        }
        De => {
            cont.text("Eigentum übergegangen auf ");
            and_join(cont, owner.iter(), |cont, org| {
                organization::link_at(cont, org.into_value(), date, site);
            });
            cont.text(".");
        }
    }
}

pub fn event_passenger(cont: &mut Content, passenger: Passenger) {
    match cont.lang() {
        En => {
            cont.text(match passenger {
                Passenger::None => "Passenger service ceased.",
                Passenger::Limited => "Limited passenger service.",
                Passenger::Historic => "Heritage passenger service.",
                Passenger::Seasonal => "Seasonal passenger service.",
                Passenger::Tourist => "Tourist passenger service.",
                Passenger::Full => "Full passenger service started.",
            })
        }
        De => {
            cont.text(match passenger {
                Passenger::None => "Personenverkehr eingestellt.",
                Passenger::Limited => "Beschränkter Personenverkehr.",
                Passenger::Historic => "Museumsbetrieb.",
                Passenger::Seasonal => "Saisonaler Personenverkehr.",
                Passenger::Tourist => "Touristischer Personenverkehr.",
                Passenger::Full => "Personenverkehr aufgenommen.",
            })
        }
    }
}

pub fn event_rails(cont: &mut Content, rails: u8) {
    match cont.lang() {
        En => {
            cont.text("Converted to ");
            match rails {
                2 => cont.text("two rail track."),
                3 => cont.text("three rail track."),
                4 => cont.text("four rail track."),
                _ => cont.text(format_args!("{}-rail track.", rails)),
            }
        }
        De => {
            cont.text("Umgestellt auf ");
            match rails {
                2 => cont.text("Zweischienengleis."),
                3 => cont.text("Dreischienengleis."),
                4 => cont.text("Vierschienengleis."),
                _ => cont.text(format_args!("{}-Schienen-Gleis.", rails)),
            }
        }
    }
}

pub fn event_status(cont: &mut Content, status: Status) {
    match cont.lang() {
        En => {
            cont.text(match status {
                Status::Planned => "Planning started.",
                Status::Construction => "Start of construction.",
                Status::Open => "Start of operation.",
                Status::Suspended => "Operation suspended.",
                Status::Reopened => "Operation restarted.",
                Status::Closed => "Permanently closed.",
                Status::Removed => "Tracks removed.",
                Status::Released => "Right of way released.",
            })
        }
        De => {
            cont.text(match status {
                Status::Planned => "Aufnahme der Planung.",
                Status::Construction => "Beginn der Bauarbeiten.",
                Status::Open => "Aufnahme des Betriebes.",
                Status::Suspended => "Zeitweilige Einstellung des Betriebes.",
                Status::Reopened => "Wiederaufnahme des Betriebes.",
                Status::Closed => "Dauerhafte Einstellung des Betriebes.",
                Status::Removed => "Abbau der Gleisanlagen.",
                Status::Released => "Entwidmung.",
            })
        }
    }
}

pub fn event_tracks(cont: &mut Content, tracks: u8) {
    match cont.lang() {
        En => {
            cont.text("Converted to ");
            match tracks{
                1 => cont.text("single track."),
                2 => cont.text("double track."),
                3 => cont.text("triple track."),
                4 => cont.text("quadruple track."),
                _ => cont.text(format_args!("{} tracks.", tracks)),
            }
        }
        De => {
            cont.text("Umgestellt auf ");
            match tracks {
                1 => cont.text("eingleisigen"),
                2 => cont.text("zweigleisigen"),
                3 => cont.text("dreigleisigen"),
                4 => cont.text("viergleisigen"),
                _ => cont.text(format_args!("{}-gleisigen", tracks)),
            }
            cont.text(" Betrieb.");
        }
    }
}

pub fn event_de_vzg(cont: &mut Content, code: &DeVzg) {
    match cont.lang() {
        En => {
            cont.text("VzG line number: ");
            cont.text(code.as_str());
            cont.text(".");
        }
        De => {
            cont.text("VzG-Nummer: ");
            cont.text(code.as_str());
            cont.text(".");
        }
    }
}

//------------ record properties ---------------------------------------------
//
// This are written as if they had the given value at the event.

pub fn record_category(cont: &mut Content, category: &Set<Category>) {
    join(cont, category.iter(), |cont, cat| cont.text(*cat))
}

pub fn record_constructor(
    cont: &mut Content,
    constructor: &List<Marked<OrganizationLink>>,
    date: &EventDate,
    site: &Site
) {
    match cont.lang() {
        En => {
            cont.text("Constructed by ");
            and_join(cont, constructor.iter(), |cont, org| {
                organization::link_at(
                    cont, org.into_value(), date, site
                );
            });
            cont.text(".");
        }
        De => {
            cont.text("Erbauer ");
            and_join(cont, constructor.iter(), |cont, org| {
                organization::link_at(
                    cont, org.into_value(), date, site
                );
            });
            cont.text(".");
        }
    }
}

pub fn record_electrified(
    cont: &mut Content,
    electrified: &Set<Electrified>,
) {
    match cont.lang() {
        En => {
            cont.text("Electric operation with ");
            and_join(cont, electrified.iter(), |c, el| c.text(el));
            cont.text(".");
        }
        De => {
            cont.text("Elektrischer Betrieb mit ");
            and_join(cont, electrified.iter(), |c, el| c.text(el));
            cont.text(".");
        }
    }
}

str_terms! {
    record_not_electrified {
        En => "Not electrified,",
        De => "Nicht elektrifiziert.",
    }
}

pub fn record_gauge(cont: &mut Content, gauge: &Set<Gauge>) {
    match cont.lang() {
        En => {
            cont.text("Gauge ");
            and_join(cont, gauge.iter(), |cont, gauge| cont.text(gauge));
        }
        De => {
            cont.text("Spurweite ");
            and_join(cont, gauge.iter(), |cont, gauge| cont.text(gauge));
        }
    }
}

pub fn record_goods(cont: &mut Content, goods: Goods) {
    match cont.lang() {
        En => {
            cont.text(match goods {
                Goods::None => "No goods service.",
                Goods::Limited => "Limited goods service only.",
                Goods::Full => "Full goods service.",
            })
        }
        De => {
            cont.text(match goods {
                Goods::None => "Ohne Güterverkehr.",
                Goods::Limited => "Mit beschränktem Güterverkehr.",
                Goods::Full => "Mit vollem Güterverkehr.",
            })
        }
    }
}

pub fn record_jurisdiction(cont: &mut Content, code: CountryCode) {
    match cont.lang() {
        En => {
            cont.text("Under jurisdiction of ");
            cont.text(code);
            cont.text(".");
        }
        De => {
            cont.text("Unter Jurisdiction von ");
            cont.text(code);
            cont.text(".");
        }
    }
}

pub fn record_name(cont: &mut Content, name: &LocalText) {
    let name = name.for_language(cont.lang().into()).unwrap_or_else(|| {
        name.first()
    });
    match cont.lang() {
        En => {
            cont.text("Name: ");
            cont.text(name);
            cont.text(".");
        }
        De => {
            cont.text("Name: ");
            cont.text(name);
            cont.text(".");
        }
    }
}

pub fn record_operator(
    cont: &mut Content,
    operator: &List<Marked<OrganizationLink>>,
    date: &EventDate,
    site: &Site
) {
    match cont.lang() {
        En => {
            cont.text("Operated by ");
            and_join(cont, operator.iter(), |cont, org| {
                organization::link_at(
                    cont, org.into_value(), date, site
                );
            });
            cont.text(".");
        }
        De => {
            cont.text("Betriebsführer ");
            and_join(cont, operator.iter(), |cont, org| {
                organization::link_at(
                    cont, org.into_value(), date, site
                );
            });
            cont.text(".");
        }
    }
}

pub fn record_owner(
    cont: &mut Content,
    owner: &List<Marked<OrganizationLink>>,
    date: &EventDate,
    site: &Site
) {
    match cont.lang() {
        En => {
            cont.text("Owned by ");
            and_join(cont, owner.iter(), |cont, org| {
                organization::link_at(
                    cont, org.into_value(), date, site
                );
            });
            cont.text(".");
        }
        De => {
            cont.text("Eigentümer ");
            and_join(cont, owner.iter(), |cont, org| {
                organization::link_at(
                    cont, org.into_value(), date, site
                );
            });
            cont.text(".");
        }
    }
}

pub fn record_passenger(cont: &mut Content, passenger: Passenger) {
    match cont.lang() {
        En => {
            cont.text(match passenger {
                Passenger::None => "No passenger service.",
                Passenger::Limited => "Limited passenger service.",
                Passenger::Historic => "Heritage passenger service.",
                Passenger::Seasonal => "Seasonal passenger service.",
                Passenger::Tourist => "Tourist passenger service.",
                Passenger::Full => "Scheduled passenger service.",
            })
        }
        De => {
            cont.text(match passenger {
                Passenger::None => "Kein Personenverkehr.",
                Passenger::Limited => "Beschränkter Personenverkehr.",
                Passenger::Historic => "Museumsbetrieb.",
                Passenger::Seasonal => "Saisonaler Personenverkehr.",
                Passenger::Tourist => "Touristischer Personenverkehr.",
                Passenger::Full => "Voller Personenverkehr.",
            })
        }
    }
}

pub fn record_rails(cont: &mut Content, rails: u8) {
    match cont.lang() {
        En => {
            match rails {
                2 => cont.text("Two rail track."),
                3 => cont.text("Three rail track."),
                4 => cont.text("Four rail track."),
                _ => cont.text(format_args!("{}-rail track.", rails)),
            }
        }
        De => {
            match rails {
                2 => cont.text("Zweischienengleis."),
                3 => cont.text("Dreischienengleis."),
                4 => cont.text("Vierschienengleis."),
                _ => cont.text(format_args!("{}-Schienen-Gleis.", rails)),
            }
        }
    }
}

pub fn record_status(cont: &mut Content, status: Status) {
    cont.text(status);
}

pub fn record_tracks(cont: &mut Content, tracks: u8) {
    match cont.lang() {
        En => {
            match tracks{
                1 => cont.text("Single track."),
                2 => cont.text("Double track."),
                3 => cont.text("Triple track."),
                4 => cont.text("Quadruple track."),
                _ => cont.text(format_args!("{} tracks", tracks)),
            }
        }
        De => {
            match tracks {
                1 => cont.text("Eingleisig."),
                2 => cont.text("Zweigleisig."),
                3 => cont.text("Dreigleisig."),
                4 => cont.text("Viergleisig."),
                _ => cont.text(format_args!("{}-gleisig.", tracks)),
            }
        }
    }
}

pub fn record_de_vzg(cont: &mut Content, code: &DeVzg) {
    match cont.lang() {
        En => {
            cont.text("VzG line number: ");
            cont.text(code.as_str());
            cont.text(".");
        }
        De => {
            cont.text("VzG-Nummer: ");
            cont.text(code.as_str());
            cont.text(".");
        }
    }
}
