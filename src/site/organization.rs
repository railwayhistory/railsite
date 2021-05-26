use raildata::document::organization;
use raildata::document::{Organization};
use crate::html;
use crate::html::target::RenderText;
use crate::http::{GetRequest, Response};
use super::Site;

impl Site {
    pub(super) fn process_organization(
        &self, mut request: GetRequest, org: &Organization
    ) -> Result<Response, GetRequest> {
        let part = match request.path_mut().next() {
            None => {
                return Ok(html::organization::details(
                    self, org, Part::Overview
                ).into_ok())
            },
            Some(part) => {
                match Part::from_str(part) {
                    Some(part) => part,
                    None => return Ok(self.not_found(request))
                }
            }
        };
        if request.path_mut().next().is_some() {
            Ok(self.not_found(request))
        }
        else if !part.is_valid(org) {
            Ok(self.not_found(request))
        }
        else {
            Ok(html::organization::details(self, org, part).into_ok())
        }
    }

    pub fn link_organization_part<'s>(
        &'s self, org: &'s Organization, part: Part
    ) -> impl RenderText + 's {
        self.link_key(org.key(), part.as_str())
    }

    pub fn link_organization_lines<'s>(
        &'s self, org: &'s Organization
    ) -> impl RenderText + 's {
        self.link_key(org.key(), Part::Lines.as_str())
    }
}


//------------ Part ----------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub enum Part {
    Overview,
    Events,
    Property,
    Lines,
}

impl Part {
    fn from_str(s: &str) -> Option<Part> {
        match s {
            "" => Some(Part::Overview),
            "events" => Some(Part::Events),
            "property" => Some(Part::Property),
            "lines" => Some(Part::Lines),
            _ => None
        }
    }

    pub fn is_valid(self, org: &Organization) -> bool {
        match self {
            Part::Lines => {
                matches!(
                    org.subtype.into_value(), organization::Subtype::Country
                )
            }
            _ => true
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Part::Overview => "",
            Part::Events => "events",
            Part::Property => "property",
            Part::Lines => "lines",
        }
    }
}

