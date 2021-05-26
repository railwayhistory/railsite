//! Translations related to organizations.

use raildata::document::OrganizationLink;
use raildata::document::organization::{
    Property, PropertyRole, Status, Subtype,
};
use raildata::types::{EventDate, List, Marked};
use crate::i18n::Lang::*;
use crate::site::Site;
use super::super::organization;
use super::super::target::{Content};
use super::utils::and_join;

str_terms! {
    term_part_overview {
        En => "Overview",
        De => "Übersicht",
    }
    term_part_events {
        En => "Events",
        De => "Ereignisse",
    }
    term_part_property {
        En => "Property",
        De => "Betriebsstrecken",
    }
    term_part_lines {
        En => "Lines",
        De => "Territoriale Strecken",
    }

    term_overview_status {
        En => "Status",
        De => "Status",
    }
    term_overview_name {
        En => "Name",
        De => "Name",
    }
    term_overview_successor {
        En => "Succeeded by",
        De => "Nachfolger",
    }
    term_overview_superior {
        En => "Part of",
        De => "Teil von",
    }

    term_not_available {
        En => "N/A",
        De => "unbekannt",
    }

    term_event_no_owner {
        En => "no owners",
        De => "keine Eigentümer",
    }

    term_property_empty {
        En => "No property lines.",
        De => "Keine Betriebsstrecken.",
    }
    term_property_line {
        En => "Line",
        De => "Strecke",
    }
    term_property_course {
        En => "Course",
        De => "Verlauf",
    }
    term_property_owner {
        En => "Owner",
        De => "Eigentümer",
    }
    term_property_operator {
        En => "Operator",
        De => "Betreiber",
    }
}

lang_enum! {
    raildata::document::organization::Status {
        En => {
            Forming => "forming",
            Open => "open",
            Closed => "closed",
        }
        De => {
            Forming => "in Gründung",
            Open => "existent",
            Closed => "aufgelöst",
        }
    }
}


//------------ Event Details -------------------------------------------------

pub fn event_capital(
    cont: &mut Content,
    domicile: &List<Marked<OrganizationLink>>,
    date: &EventDate,
    site: &Site
) {
    match cont.lang() {
        En => {
            cont.text("Capital ");
            and_join(cont, domicile.iter(), |cont, dom| {
                organization::link_at(cont, dom.into_value(), date, site);
            });
        }
        De => {
            cont.text("Hauptstadt ");
            and_join(cont, domicile.iter(), |cont, dom| {
                organization::link_at(cont, dom.into_value(), date, site);
            });
        }
    }
}

pub fn event_domicile(
    cont: &mut Content,
    domicile: &List<Marked<OrganizationLink>>,
    date: &EventDate,
    site: &Site
) {
    match cont.lang() {
        En => {
            cont.text("Domicile: ");
            and_join(cont, domicile.iter(), |cont, dom| {
                organization::link_at(cont, dom.into_value(), date, site);
            });
        }
        De => {
            cont.text("Domizil: ");
            and_join(cont, domicile.iter(), |cont, dom| {
                organization::link_at(cont, dom.into_value(), date, site);
            });
        }
    }
}

pub fn event_owner(
    cont: &mut Content,
    owner: &List<Marked<OrganizationLink>>,
    date: &EventDate,
    site: &Site,
) {
    match owner.len() {
        0 => cont.text(term_event_no_owner),
        1 => event_one_owner(cont, owner.first().unwrap(), date, site),
        _ => event_many_owners(cont, owner, date, site),
    }
}

pub fn event_one_owner(
    cont: &mut Content,
    owner: &Marked<OrganizationLink>,
    date: &EventDate,
    site: &Site,
) {
    match cont.lang() {
        En => {
            cont.text("Owned by ");
            organization::link_at(
                cont, owner.into_value(), date, site
            );
        }
        De => {
            cont.text("Eigentümer: ");
            organization::link_at(
                cont, owner.into_value(), date, site
            );
        }
    }
}

pub fn event_many_owners(
    cont: &mut Content,
    owner: &List<Marked<OrganizationLink>>,
    date: &EventDate,
    site: &Site,
) {
    match cont.lang() {
        En => {
            cont.text("Owners: ");
            and_join(cont, owner.iter(), |cont, org| {
                organization::link_at(
                    cont, org.into_value(), date, site
                );
            });
            cont.text(".");
        }
        De => {
            cont.text("Eigentümer: ");
            and_join(cont, owner.iter(), |cont, org| {
                organization::link_at(cont, org.into_value(), date, site);
            });
            cont.text(".");
        }
    }
}

pub fn event_property(
    cont: &mut Content,
    property: &Property,
    date: &EventDate,
    site: &Site
) {
    match cont.lang() {
        En => {
            match property.role.into_value() {
                PropertyRole::Constructor => {
                    cont.text("For lines under construction")
                }
                PropertyRole::Owner => {
                    cont.text("For owned lines")
                }
                PropertyRole::Operator => {
                    cont.text("Operated lines")
                }
            }
            cont.text(" transfered ");
            let mut some = false;
            if !property.owner.is_empty() {
                cont.text("ownership to ");
                and_join(cont, property.owner.iter(), |cont, org| {
                    organization::link_at(
                        cont, org.into_value(), date, site
                    );
                });
                some = true;
            }
            if !property.operator.is_empty() {
                if some {
                    cont.text("; ");
                }
                cont.text("operation to ");
                and_join(cont, property.operator.iter(), |cont, org| {
                    organization::link_at(
                        cont, org.into_value(), date, site
                    );
                });
                some = true;
            }
            if !property.constructor.is_empty() {
                if some {
                    cont.text("; ");
                }
                cont.text("construction to ");
                and_join(cont, property.constructor.iter(), |cont, org| {
                    organization::link_at(
                        cont, org.into_value(), date, site
                    );
                });
            }
            cont.text(".");
        }
        De => {
            match property.role.into_value() {
                PropertyRole::Constructor => {
                    cont.text("Für Strecken in Bau")
                }
                PropertyRole::Owner => {
                    cont.text("Für Strecken in Eigentum")
                }
                PropertyRole::Operator => {
                    cont.text("Für betriebene Strecken")
                }
            }
            cont.text(" Übergabe ");
            let mut some = false;
            if !property.owner.is_empty() {
                cont.text("des Eigentums an ");
                and_join(cont, property.owner.iter(), |cont, org| {
                    organization::link_at(
                        cont, org.into_value(), date, site
                    );
                });
                some = true;
            }
            if !property.operator.is_empty() {
                if some {
                    cont.text("; ");
                }
                cont.text("der Betriebsführung an ");
                and_join(cont, property.operator.iter(), |cont, org| {
                    organization::link_at(
                        cont, org.into_value(), date, site
                    );
                });
                some = true;
            }
            if !property.constructor.is_empty() {
                if some {
                    cont.text("; ");
                }
                cont.text("des Baus an ");
                and_join(cont, property.constructor.iter(), |cont, org| {
                    organization::link_at(
                        cont, org.into_value(), date, site
                    );
                });
            }
            cont.text(".");
        }
    }
}

pub fn event_status(
    cont: &mut Content,
    status: Status,
    subtype: Subtype
) {
    match cont.lang() {
        En => {
            match status {
                Status::Forming => cont.text("forming"),
                Status::Open => {
                    match subtype {
                        Subtype::Person => cont.text("born"),
                        _ => cont.text("established")
                    }
                }
                Status::Closed => {
                    match subtype {
                        Subtype::Person => cont.text("died"),
                        _ => cont.text("dissolved")
                    }
                }
            }
        }
        De => {
            match status {
                Status::Forming => cont.text("Beginn Gründungsvorbereitung"),
                Status::Open => { 
                    match subtype {
                        Subtype::Person => cont.text("geboren"),
                        _ => cont.text("Gründung")
                    }
                }
                Status::Closed => {
                    match subtype {
                        Subtype::Person => cont.text("gestorben"),
                        _ => cont.text("aufgelöst")
                    }
                }
            }
        }
    }
}

pub fn event_successor(
    cont: &mut Content,
    superior: OrganizationLink,
    date: &EventDate,
    site: &Site
) {
    cont.text(term_overview_successor);
    cont.text(" ");
    organization::link_at(cont, superior, date, site);
}

pub fn event_superior(
    cont: &mut Content,
    superior: OrganizationLink,
    date: &EventDate,
    site: &Site
) {
    cont.text(term_overview_superior);
    cont.text(" ");
    organization::link_at(cont, superior, date, site);
}

