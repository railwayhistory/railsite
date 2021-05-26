/*
use raildata::document::{Organization};
use crate::http::{GetRequest, Response};
use super::Site;

impl Site {
    pub(super) fn process_country(
        &self, mut _request: GetRequest, _organization: &Organization
    ) -> Response {
        unimplemented!()
        /*
        match request.path_mut().next_and_last() {
            Ok(None) => {
                self.process_country_summary(organization)
            }
            Ok(Some("lines")) => {
                self.process_country_lines(organization)
            }
            _ => self.not_found(request),
        }
        */
    }

    /*
    pub fn link_country_lines<'a>(
        &'a self, organization: &'a Organization
    ) -> impl Template + 'a {
        self.link(owned_html! {
            : "key/";
            : organization.key().as_str();
            : "/lines/";
        })
    }

    fn process_country_summary(
        &self, _organization: &Organization
    ) -> Response {
        unimplemented!()
    }

    fn process_country_lines(
        &self, organization: &Organization
    ) -> Response {
        html2::country::Lines::new(self, organization).ok()
    }
    */
}
*/

