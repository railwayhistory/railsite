use raildata::document::Source;
use crate::html;
use crate::html::target::RenderText;
use crate::http::{GetRequest, Response};
use super::Site;

impl Site {
    pub(super) fn process_source(
        &self, mut request: GetRequest, source: &Source
    ) -> Result<Response, GetRequest> {
        let part = match request.path_mut().next() {
            None => {
                return Ok(html::source::details(
                    self, source, Part::Overview
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
            Ok(html::source::details(self, source, part).into_ok())
        }
    }

    pub fn link_source_part<'s>(
        &'s self, source: &'s Source, part: Part
    ) -> impl RenderText + 's {
        self.link_key(source.key(), part.as_str())
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


