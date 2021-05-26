use horrorshow::{html, TemplateBuffer};
use crate::site::Site;
use super::core::Page;
use super::nav;
use crate::documentation;


//------------ DocPage -------------------------------------------------------

pub struct DocPage<'a> {
    site: &'a Site,
    page: &'a documentation::Page,
}

impl<'a> DocPage<'a> {
    pub fn new(site: &'a Site, page: &'a documentation::Page) -> Self {
        DocPage { site, page }
    }
}

impl<'a> Page for DocPage<'a> {
    type Nav = nav::Other;

    fn site(&self) -> &Site { &self.site }

    fn title(&self, tmpl: &mut TemplateBuffer) {
        write!(tmpl, "{}", self.page.title);
    }

    fn content(&self, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            div {
                |tmpl| tmpl.write_raw(&self.page.content);
            }
        }
    }
}


