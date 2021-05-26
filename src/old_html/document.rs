use horrorshow::{html, TemplateBuffer};
use raildata::document::Document;
use crate::site::Site;
use super::core::Page;
use super::nav;


//------------ Summary -------------------------------------------------------

/// The summary page for a document of any kind.
pub struct Summary<'a> {
    site: &'a Site,
    document: &'a Document,
}

impl<'a> Summary<'a> {
    pub fn new(site: &'a Site, document: &'a Document) -> Self {
        Summary { site, document }
    }
}

impl<'a> Page for Summary<'a> {
    type Nav = nav::Other;

    fn site(&self) -> &Site { &self.site }

    fn title(&self, tmpl: &mut TemplateBuffer) {
        write!(tmpl, "{}", self.document.key());
    }

    fn content(&self, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            pre {
                |tmpl| write!(tmpl, "{:#?}", self.document);
            }
        }
    }
}

