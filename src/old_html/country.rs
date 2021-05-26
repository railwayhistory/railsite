use std::iter;
use std::str::FromStr;
use horrorshow::{html, TemplateBuffer};
use raildata::document::{Line, Organization, Point};
use raildata::types::CountryCode;
use crate::site::Site;
use super::core::Page;
use super::nav;


//------------ Lines ---------------------------------------------------------

/// The summary page for a document of any kind.
pub struct Lines<'a> {
    site: &'a Site,
    organization: &'a Organization,
    code: CountryCode,
}

impl<'a> Lines<'a> {
    pub fn new(site: &'a Site, organization: &'a Organization) -> Self {
        let key = organization.key().as_str();
        let key = if key.starts_with("org.") {
            &key[4..]
        }
        else {
            &"xx"
        };
        let code = CountryCode::from_str(key).unwrap_or(CountryCode::INVALID);
        Lines { site, organization, code }
    }

    pub fn organization(&self) -> &Organization {
        self.organization
    }

    fn lines<'s>(&'s self) -> impl Iterator<Item = CountryLine<'a>> + 's {
        self.site.catalogue().country_lines().by_code(
            self.code
        ).iter().map(move |link| {
            CountryLine {
                site: self.site,
                line: link.follow(self.site.library())
            }
        })
    }
}

impl<'a> Page for Lines<'a> {
    type Nav = nav::Other;

    fn site(&self) -> &Site { &self.site }

    /*
    fn title(&self, tmpl: &mut TemplateBuffer) {
        write!(tmpl, "{}", self.document.key());
    }
    */

    fn content(&self, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            table(class = "table") {
                @for line in self.lines() {
                    tr {
                        td {
                            : line.code()
                        }
                        td {
                            @for (point, first) in line.junctions() {
                                @if !first {
                                    : " – "
                                }
                                : point.name(Some(self.code));
                            }
                        }
                    }
                }    
            }
        }
    }
}


//------------ CountryLine ---------------------------------------------------

struct CountryLine<'a> {
    site: &'a Site,
    line: &'a Line,
}

impl<'a> CountryLine<'a> {
    fn code(&self) -> &str {
        self.line.code().1
    }

    fn junctions<'s>(&'s self) -> impl Iterator<Item = (&'a Point, bool)> + 's {
        self.line.junctions(self.site.library()).zip(
            iter::once(true).chain(iter::repeat(false))
        )
    }
}

