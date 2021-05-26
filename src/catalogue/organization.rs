//! Information related to organizations.

use std::collections::HashMap;
use raildata::document::{
    Document, DocumentLink, LineLink, OrganizationLink
};
use raildata::document::organization::Subtype;
use raildata::library::Library;
use raildata::types::{CountryCode, Key};
use crate::i18n::Lang;


//------------ CountryIndex --------------------------------------------------

/// The index of countries.
#[derive(Clone, Debug, Default)]
pub struct CountryIndex {
    countries: HashMap<Lang, Vec<OrganizationLink>>,
}

impl CountryIndex {
    pub(super) fn insert(
        &mut self, org: &Document, link: DocumentLink
    ) {
        if let Some(org) = org.try_as_organization() {
            if matches!(org.subtype.into_value(), Subtype::Country) {
                for lang in Lang::all() {
                    self.countries.entry(lang).or_default().push(link.into())
                }
            }
        }
    }

    pub(super) fn finalize(&mut self, library: &Library) {
        for (&lang, list) in self.countries.iter_mut() {
            list.sort_by_key(|link| {
                link.follow(library).local_short_name(lang.into())
            })
        }

        // Make sure we have entries for all languages in case nothing was
        // actually inserted. This way we can unwrap below.
        for lang in Lang::all() {
            self.countries.entry(lang).or_default();
        }
    }

    pub fn iter(
        &self, lang: Lang
    ) -> impl Iterator<Item = OrganizationLink> + '_ {
        self.countries.get(&lang).unwrap().iter().cloned()
    }
}


//------------ CountryLines --------------------------------------------------

/// An index of lines for each country.
///
/// The index is based on the country code portion of the line code. For each
/// country, the lines are ordered by their code.
#[derive(Clone, Debug, Default)]
pub struct CountryLines {
    countries: HashMap<CountryCode, Option<OrganizationLink>>,
    index: HashMap<OrganizationLink, Vec<LineLink>>,
}

impl CountryLines {
    pub(super) fn insert(
        &mut self, document: &Document, link: DocumentLink, library: &Library
    ) {
        let line = match document.try_as_line() {
            Some(line) => line,
            None => return,
        };

        let code = match line.country() {
            Some(code) => code,
            None => return,
        };

        let org = match self.countries.entry(code).or_insert_with(|| {
            let mut key = String::with_capacity(6);
            key.push_str("org.");
            for ch in code.as_str().chars() {
                key.push(ch.to_ascii_lowercase())
            }
            let key = Key::from_string(key).unwrap();
            library.get(&key).map(Into::into)
        }) {
            Some(org) => *org,
            None => return
        };
        self.index.entry(org).or_default().push(link.into());
    }

    pub(super) fn finalize(&mut self, library: &Library) {
        for lines in self.index.values_mut() {
            lines.sort_unstable_by(|left, right| {
                left.follow(library).key().cmp(
                    right.follow(library).key()
                )
            })
        }
    }

    pub fn by_link(&self, link: OrganizationLink) -> &[LineLink] {
        self.index.get(&link).map(Vec::as_slice).unwrap_or_default()
    }

    pub fn by_code(&self, code: CountryCode) -> &[LineLink] {
        match self.countries.get(&code) {
            Some(Some(link)) => self.by_link(*link),
            _ => &[]
        }
    }
}


//------------ PropertyLines -------------------------------------------------

#[derive(Clone, Debug, Default)]
pub struct PropertyLines {
    lines: HashMap<OrganizationLink, Vec<PropertyLine>>,
}

#[derive(Clone, Copy, Debug)]
pub struct PropertyLine {
    pub line: LineLink,
    pub owned: Option<bool>,
    pub operated: Option<bool>,
}

impl PropertyLines {
    pub(super) fn insert(
        &mut self, document: &Document, link: DocumentLink
    ) {
        let line = match document.try_as_line() {
            Some(line) => line,
            None => return,
        };
        let link = link.into();

        let mut orgs = HashMap::new();

        for event in &line.events {
            if let Some(ref owner_list) = event.owner {
                for org in owner_list {
                    orgs.entry(
                        org.into_value()
                    ).or_insert((None, None)).0 = Some(true);
                }
            }
            if let Some(ref list) = event.operator {
                for org in list {
                    orgs.entry(
                        org.into_value()
                    ).or_insert((None, None)).1 = Some(true);
                }
            }
            if let Some(ref concession) = event.concession {
                for org in &concession.to {
                    orgs.entry(
                        org.into_value()
                    ).or_insert((None, None));
                }
            }
        }

        for (org, (owned, operated)) in orgs {
            self.lines.entry(org).or_default().push(
                PropertyLine { line: link, owned, operated }
            )
        }
    }

    pub(super) fn finalize(&mut self, library: &Library) {
        for list in self.lines.values_mut() {
            list.sort_unstable_by(|left, right| {
                left.line.follow(library).key().cmp(
                    right.line.follow(library).key()
                )
            })
        }
    }

    pub fn by_link(&self, link: OrganizationLink) -> &[PropertyLine] {
        self.lines.get(&link).map(Vec::as_slice).unwrap_or_default()
    }
}

