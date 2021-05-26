use crate::http::{GetRequest, Response};
use super::Site;

//------------ Site ----------------------------------------------------------

impl Site {
    pub(super) fn process_doc(
        &self, request: GetRequest
    ) -> Result<Response, GetRequest> {
        Err(request)
    }

    /*
        if request.path().segment() != "docs" {
            return Err(request)
        }

        if request.path_mut().next().is_none() {
            self.process_start_page(request)
        }
        else if request.path().segment() == "topic" {
            self.process_doc_topic(request)
        }
        else {
            self.process_doc_page(request)
        }
    }

    fn process_doc_topic(
        &self, mut request: Request
    ) -> Result<Response, Request> {
        if request.path_mut().next().is_none() {
            return Ok(self.not_found(request))
        }
        let topic = request.path().segment();

        match self.documentation().get_topic_path(self.lang().into(), topic) {
            Some(path) => {
                Ok(request.redirect(
                    self.link_page(path, Some(topic))
                        .into_string()
                        .unwrap()
                ))
            }
            None => {
                Ok(self.not_found(request))
            }
        }
    }

    fn process_doc_page(
        &self, request: Request
    ) -> Result<Response, Request> {
        match self.documentation().get_page(
            self.lang().into(), request.path().remaining()
        ) {
            Some(page) => Ok(html2::doc::DocPage::new(self, page).ok()),
            None => Ok(self.not_found(request))
        }
    }

    fn process_start_page(
        &self, request: Request
    ) -> Result<Response, Request> {
        match self.documentation().get_page(
            self.lang().into(), ""
        ) {
            Some(page) => Ok(html2::doc::DocPage::new(self, page).ok()),
            None => Ok(self.not_found(request))
        }
    }

    pub fn link_docs<'a>(
        &'a self
    ) -> impl Template + 'a {
        self.link("docs/")
    }

    pub fn link_topic<'a>(
        &'a self, topic: &'a str
    ) -> impl Template + 'a {
        match self.documentation().get_topic_path(self.lang().into(), topic) {
            Some(path) => {
                self.link_page(path, Some(topic))
            }
            None => {
                self.link_page("", None)
            }
        }
    }

    fn link_page<'a>(
        &'a self, page: &'a str, fragment: Option<&'a str>
    ) -> impl Template + 'a {
        self.link(owned_html! {
            : "doc/";
            : page;
            @ if let Some(fragment) = fragment {
                : "#";
                : fragment
            }
        })
    }
*/
}

