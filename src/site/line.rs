use raildata::document::line::Line;
use crate::html;
use crate::html::target::RenderText;
use crate::http::{GetRequest, Response};
use super::Site;

impl Site {
    pub(super) fn process_line(
        &self, mut request: GetRequest, line: &Line,
    ) -> Result<Response, GetRequest> {
        let part = match request.path_mut().next() {
            None => {
                return Ok(html::line::details(
                    self, line, Part::Overview
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
            Ok(html::line::details(self, line, part).into_ok())
        }
    }

    pub fn link_line_part<'s>(
        &'s self, line: &'s Line, part: Part
    ) -> impl RenderText + 's {
        self.link_key(line.key(), part.as_str())
    }
}


//------------ Part ----------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub enum Part {
    Overview,
    Route,
    Events,
    Records,
}

impl Part {
    fn from_str(s: &str) -> Option<Part> {
        match s {
            "" => Some(Part::Overview),
            "route" => Some(Part::Route),
            "events" => Some(Part::Events),
            "records" => Some(Part::Records),
            _ => None
        }
    }
 
    fn as_str(self) -> &'static str {
        match self {
            Part::Overview => "",
            Part::Route => "route",
            Part::Events => "events",
            Part::Records => "records",
        }
    }
}

