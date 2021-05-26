use raildata::document::Point;
use crate::html;
use crate::html::target::RenderText;
use crate::http::{GetRequest, Response};
use super::Site;

impl Site {
    pub(super) fn process_point(
        &self, mut request: GetRequest, point: &Point
    ) -> Result<Response, GetRequest> {
        let part = match request.path_mut().next() {
            None => {
                return Ok(html::point::details(
                    self, point, Part::Overview
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
            Ok(html::point::details(self, point, part).into_ok())
        }
    }

    pub fn link_point_part<'s>(
        &'s self, point: &'s Point, part: Part
    ) -> impl RenderText + 's {
        self.link_key(point.key(), part.as_str())
    }
}


//------------ Part ----------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub enum Part {
    Overview,
    Events,
    Records,
}

impl Part {
    fn from_str(s: &str) -> Option<Part> {
        match s {
            "" => Some(Part::Overview),
            "events" => Some(Part::Events),
            "records" => Some(Part::Records),
            _ => None
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Part::Overview => "",
            Part::Events => "events",
            Part::Records => "records"
        }
    }
}

