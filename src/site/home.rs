use std::fmt;
use crate::html;
use crate::http::{GetRequest, Response};
use super::Site;

//------------ Site ----------------------------------------------------------

impl Site {
    pub(super) fn process_home(
        &self, _request: GetRequest
    ) -> Result<Response, GetRequest> {
        if request.path().segment().is_empty() {
        }
        else {
            Err(request)
        }
    }

    /*
        if request.path().segment().is_empty() {
            Ok(html2::index::Home::new(self).ok())
        }
        else if request.path().segment() == "lines" {
            Ok(self.process_line_index(request))
        }
        else {
            Err(request)
        }
    }
    */

    pub fn link_home<'s>(&'s self) -> impl fmt::Display + 's {
        self.link("/")
    }

    /*
    pub fn link_line_index<'a>(&'a self) -> impl Template + 'a {
        self.link("/lines")
    }

    fn process_line_index(&self, request: Request) -> Response {
        html2::index::Lines::new(self, &request).ok()
    }
    */
}

