use raildata::document::{Document, Line, Organization, Point, Source};
use raildata::library::LinkTarget;
use raildata::types::Key;
use crate::html::target::{RenderText, Text};
use crate::http::{GetRequest, Response};
use super::Site;

impl Site {
    pub(super) fn process_document(
        &self, mut request: GetRequest
    ) -> Result<Response, GetRequest> {
        if request.path().segment() != "key" {
            return Err(request)
        }

        if request.path_mut().next().is_none() {
            return Ok(self.not_found(request))
        }

        let key = match Key::from_string(request.path().segment().into()) {
            Ok(key) => key,
            Err(_) => {
                return Ok(self.not_found(request))
            }
        };

        let link = match self.library().get(&key) {
            Some(link) => link,
            None => {
                return Ok(self.not_found(request))
            }
        };

        let document = self.library().resolve(link);

        match *document {
            Document::Line(ref line) => {
                self.process_line(request, line)
            }
            Document::Organization(ref org) => {
                self.process_organization(request, org)
            }
            Document::Point(ref point) => {
                self.process_point(request, point)
            }
            Document::Source(ref source) => {
                self.process_source(request, source)
            }
            Document::Structure(ref structure) => {
                self.process_structure(request, structure)
            }
            _ => {
                Ok(self.not_found(request))
            }
        }
    }

    pub(super) fn link_key<'s>(
        &'s self, key: &'s Key, path: &'s str
    ) -> impl RenderText + 's {
        self.link(KeyPath(key, path))
    }

    pub fn link_document<'s>(
        &'s self, document: &'s Document
    ) -> impl RenderText + 's {
        self.link_key(document.key(), "")
    }

    pub fn link_line<'s>(
        &'s self, line: &'s Line
    ) -> impl RenderText + 's {
        self.link_key(line.key(), "")
    }

    pub fn link_point<'s>(
        &'s self, point: &'s Point
    ) -> impl RenderText + 's {
        self.link_key(point.key(), "")
    }

    pub fn link_organization<'s>(
        &'s self, org: &'s Organization
    ) -> impl RenderText + 's {
        self.link_key(org.key(), "")
    }

    pub fn link_source<'s>(
        &'s self, source: &'s Source
    ) -> impl RenderText + 's {
        self.link_key(source.key(), "")
    }
}


//------------ KeyPath -------------------------------------------------------

pub struct KeyPath<'a>(&'a Key, &'a str);

impl<'a> RenderText for KeyPath<'a> {
    fn render(self, target: &mut Text) {
        target.push_str("key/");
        target.push_str(self.0.as_str());
        target.push_str("/");
        target.push_str(self.1);
    }
}

