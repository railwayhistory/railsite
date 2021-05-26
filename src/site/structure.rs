use raildata::document::Structure;
use crate::html;
use crate::html::target::RenderText;
use crate::http::{GetRequest, Response};
use super::Site;


impl Site {
    pub(super) fn process_structure(
        &self, mut request: GetRequest, structure: &Structure,
    ) -> Result<Response, GetRequest> {
        let part = match request.path_mut().next() {
            None => {
                return Ok(html::structure::details(
                    self, structure, Part::Overview
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
        else {
            Ok(html::structure::details(self, structure, part).into_ok())
        }
    }

    pub fn link_structure_part<'s>(
        &'s self, structure: &'s Structure, part: Part
    ) -> impl RenderText + 's {
        self.link_key(structure.key(), part.as_str())
    }
}


//------------ Part ----------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub enum Part {
    Overview,
}

impl Part {
    fn from_str(s: &str) -> Option<Part> {
        match s {
            "" => Some(Part::Overview),
            _ => None
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Part::Overview => "",
        }
    }
}


