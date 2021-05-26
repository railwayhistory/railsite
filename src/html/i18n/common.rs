
use raildata::document::common::{
    Agreement, AgreementType, Alternative, Basis
};
use raildata::types::{CountryCode};
use raildata::types::date::{Date, EventDate, Precision};
use raildata::types::local::LocalCode;
use crate::i18n::Lang::*;
use crate::site::Site;
use super::super::{i18n, organization};
use super::super::target::{Content, RenderText, Text};
use super::utils::{and_join, capitalize};

str_terms! {
    term_not_available {
        En => "N/A",
        De => "nicht bekannt",
    }
}

//------------ Agreement -----------------------------------------------------

pub fn agreement_base(
    content: &mut Content,
    agreement: &Agreement,
    date: &EventDate,
    site: &Site, 
) {
    match content.lang() {
        En => {
            content.text(agreement.agreement_type);
            content.text(" between ");
            and_join(content, agreement.parties.iter(), |cont, org| {
                organization::link_at(cont, org.into_value(), date, site);
            })
        }
        De => {
            content.text(agreement.agreement_type);
            content.text(" zwischen ");
            and_join(content, agreement.parties.iter(), |cont, org| {
                organization::link_at(cont, org.into_value(), date, site);
            })
        }
    }
}


//------------ AgreementType -------------------------------------------------

lang_enum! {
    AgreementType {
        En => {
            Contract => "Contract",
            Treaty => "Treaty",
        }
        De => {
            Contract => "Vertrag",
            Treaty => "Staatsvertrag",
        }
    }
}


//------------ Alternative ---------------------------------------------------

pub fn alternative_base(alternative: &Alternative, content: &mut Content) {
    match content.lang() {
        En => {
            content.text("Alternatively at ");
            content.text(&alternative.date);
            content.text(".");
        }
        De => {
            content.text("Alternativ ");
            content.text(&alternative.date);
            content.text(".");
        }
    }
}


//------------ Basis ---------------------------------------------------------

pub fn basis_base(cont: &mut Content, basis: &Basis, site: &Site) {
    match cont.lang() {
        En => {
            cont.text("Based on ");
            if let Some(ref agreement) = basis.agreement {
                cont.text(match agreement.agreement_type {
                    AgreementType::Contract => "a contract",
                    AgreementType::Treaty => "a treaty",
                });
                if !basis.date.is_empty() {
                    cont.text(" of ");
                    cont.text(&basis.date);
                }
                cont.text(" between ");
                and_join(cont, agreement.parties.iter(), |cont, org| {
                    organization::link_at(
                        cont, org.into_value(), &basis.date, site
                    );
                })
            }
            else {
                cont.text("a document");
                if !basis.date.is_empty() {
                    cont.text(" from ");
                    cont.text(&basis.date);
                }
            }
        }
        De => {
            cont.text("Auf Grundlage ");
            if let Some(ref agreement) = basis.agreement {
                cont.text(match agreement.agreement_type {
                    AgreementType::Contract => "eines Vertrages",
                    AgreementType::Treaty => "eines Staatsvertrages",
                });
                if !basis.date.is_empty() {
                    cont.text(" vom ");
                    cont.text(&basis.date);
                }
                cont.text(" zwischen ");
                and_join(cont, agreement.parties.iter(), |cont, org| {
                    organization::link_at(
                        cont, org.into_value(), &basis.date, site
                    );
                })
            }
            else {
                cont.text("eines Dokuments");
                if !basis.date.is_empty() {
                    cont.text(" vom ");
                    cont.text(&basis.date);
                }
            }
        }
    }
}


//------------ CountryCode ---------------------------------------------------

impl RenderText for CountryCode {
    fn render(self, text: &mut Text) {
        match text.lang() {
            En => {
                text.push_str(match self {
                    CountryCode::DE => "Germany",
                    CountryCode::RU => "Russia",
                    _ => self.as_str()
                })
            }
            De => {
                text.push_str(match self {
                    CountryCode::DE => "Deutschland",
                    CountryCode::RU => "Russland",
                    _ => self.as_str()
                })
            }
        }
    }
}


//------------ Date and EventDate --------------------------------------------

impl<'a> RenderText for &'a Date {
    fn render(self, text: &mut Text) {
        match text.lang() {
            En => {
                match self.precision() {
                    Precision::Exact => { }
                    Precision::Circa => { text.push_str("c.\u{202f}") }
                    Precision::Before => { text.push_str("<\u{202f}") }
                    Precision::After => { text.push_str(">\u{202f}") }
                }
                write!(text, "{}", self.year());
                if let Some(month) = self.month() {
                    write!(text, "-{:02}", month);
                    if let Some(day) = self.day() {
                        write!(text, "-{:02}", day);
                    }
                }
                if self.doubt() {
                    text.push_str("\u{202f}?");
                }
            }
            De => {
                match self.precision() {
                    Precision::Exact => { }
                    Precision::Circa => { text.push_str("ca.\u{202f}") }
                    Precision::Before => { text.push_str("<\u{202f}") }
                    Precision::After => { text.push_str(">\u{202f}") }
                }
                if let Some(month) = self.month() {
                    if let Some(day) = self.day() {
                        write!(text, "{}.\u{202f}", day);
                    }
                    text.push_str(roman_month(month));
                    text.push('\u{202f}');
                }
                write!(text, "{}", self.year());
                if self.doubt() {
                    text.push_str("\u{202f}?");
                }
            }
        }
    }
}

impl<'a> RenderText for &'a EventDate {
    fn render(self, text: &mut Text) {
        if self.is_empty() {
            //text.push('\u{2014}');
        }
        else if self.len() == 1 {
            self.as_slice()[0].as_value().render(text);
        }
        else {
            let mut first = true;
            for date in self.iter() {
                if first {
                    first = false;
                }
                else {
                    text.push_str(", ");
                }
                date.as_value().render(text);
            }
        }
    }
}

fn roman_month(month: u8) -> &'static str {
    match month {
        1 => "I.",
        2 => "II.",
        3 => "III.",
        4 => "IV.",
        5 => "V.",
        6 => "VI.",
        7 => "VII.",
        8 => "VIII.",
        9 => "IX.",
        10 => "X.",
        11 => "XI.",
        12 => "XII.",
        _ => unreachable!(),
    }
}


//------------ Event Details -------------------------------------------------

pub fn event_name(
    cont: &mut Content,
    name: &str
) {
    match cont.lang() {
        En => {
            cont.text("Name: ");
            cont.text(name)
        }
        De => {
            cont.text("Name: ");
            cont.text(name)
        }
    }
}

pub fn event_local_name(
    cont: &mut Content,
    code: LocalCode,
    name: &str
) {
    match cont.lang() {
        En => {
            match i18n::local::en::local_adjective(code) {
                Some(word) => {
                    cont.text(word);
                    cont.text(" name: ");
                }
                None => {
                    cont.text(code.as_str());
                    cont.text("-name: ");
                }
            }
            cont.text(name);
        }
        De => {
            match i18n::local::de::masculine_local_adjective(code) {
                Some(word) => {
                    cont.text(capitalize(word));
                    cont.text(" Name: ");
                }
                None => {
                    cont.text(code.as_str());
                    cont.text("-Name: ");
                }
            }
            cont.text(name)
        }
    }
}

pub fn event_short_name(
    cont: &mut Content,
    name: &str
) {
    match cont.lang() {
        En => {
            cont.text("Short name: ");
            cont.text(name)
        }
        De => {
            cont.text("Verkürzter Name: ");
            cont.text(name)
        }
    }
}

pub fn event_local_short_name(
    cont: &mut Content,
    code: LocalCode,
    name: &str
) {
    match cont.lang() {
        En => {
            match i18n::local::en::local_adjective(code) {
                Some(word) => {
                    cont.text(word);
                    cont.text(" short name: ");
                }
                None => {
                    cont.text("Short name (");
                    cont.text(code.as_str());
                    cont.text("): ");
                }
            }
            cont.text(name);
        }
        De => {
            match i18n::local::de::masculine_local_adjective(code) {
                Some(word) => {
                    cont.text(capitalize(word));
                    cont.text(" verkürzter Name: ");
                }
                None => {
                    cont.text("Verkürzter Name (");
                    cont.text(code.as_str());
                    cont.text("): ");
                }
            }
            cont.text(name)
        }
    }
}

pub fn event_public_name(
    cont: &mut Content,
    name: &str
) {
    match cont.lang() {
        En => {
            cont.text("Public name: ");
            cont.text(name)
        }
        De => {
            cont.text("Öffentlicher Name: ");
            cont.text(name)
        }
    }
}

pub fn event_local_public_name(
    cont: &mut Content,
    code: LocalCode,
    name: &str
) {
    match cont.lang() {
        En => {
            match i18n::local::en::local_adjective(code) {
                Some(word) => {
                    cont.text(word);
                    cont.text(" public name: ");
                }
                None => {
                    cont.text("Public name (");
                    cont.text(code.as_str());
                    cont.text("): ");
                }
            }
            cont.text(name);
        }
        De => {
            match i18n::local::de::masculine_local_adjective(code) {
                Some(word) => {
                    cont.text(capitalize(word));
                    cont.text(" öffentlicher Name: ");
                }
                None => {
                    cont.text("Öffentlicher Name (");
                    cont.text(code.as_str());
                    cont.text("): ");
                }
            }
            cont.text(name)
        }
    }
}

pub fn event_designation(
    cont: &mut Content,
    name: &str
) {
    match cont.lang() {
        En => {
            cont.text("Inofficial designation: ");
            cont.text(name)
        }
        De => {
            cont.text("Inoffizielle Bezeichnung: ");
            cont.text(name)
        }
    }
}

pub fn event_local_designation(
    cont: &mut Content,
    code: LocalCode,
    name: &str
) {
    match cont.lang() {
        En => {
            match i18n::local::en::local_adjective(code) {
                Some(word) => {
                    cont.text(word);
                    cont.text(" inofficial designation: ");
                }
                None => {
                    cont.text(code.as_str());
                    cont.text("Inofficial designation (");
                    cont.text(code.as_str());
                    cont.text("): ");
                }
            }
            cont.text(name);
        }
        De => {
            match i18n::local::de::feminine_local_adjective(code) {
                Some(word) => {
                    cont.text(capitalize(word));
                    cont.text(" inoffizielle Bezeichnung: ");
                }
                None => {
                    cont.text("Inoffizielle Bezeichnung (");
                    cont.text(code.as_str());
                    cont.text("): ");
                }
            }
            cont.text(name)
        }
    }
}

