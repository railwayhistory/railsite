use horrorshow::{html, TemplateBuffer};
use hyper::Method;
use crate::i18n::Lang;
use crate::site::Site;
use super::core::Page;
use super::nav;


//------------ NotFound ------------------------------------------------------

/// Page not found error.
pub struct NotFound<'a> {
    site: &'a Site,
    path: &'a str,
}

impl<'a> NotFound<'a> {
    pub fn new(site: &'a Site, path: &'a str) -> Self {
        NotFound { site, path }
    }
}

impl<'a> Page for NotFound<'a> {
    type Nav = nav::Other;

    fn site(&self) -> &Site { self.site }

    fn title(&self, tmpl: &mut TemplateBuffer) {
        if self.site.lang() == Lang::De {
            tmpl.write_str("404. Nicht gefunden");
        }
        else {
            tmpl.write_str("404. Not Found");
        }
    }

    fn header(&self, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            h1 {
                @ if self.site.lang() == Lang::De {
                    : "404 Nicht gefunden"
                }
                else {
                    : "404 Not Found"
                }
            }
        }
    }

    fn content(&self, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            p {
                @ if self.site.lang() == Lang::De {
                    : "Der Pfad ";
                    tt { : self.path }
                    : " konnte auf dem Server nicht gefunden werden."
                }
                else {
                    : "Path ";
                    tt { : self.path }
                    : " not found on this server."
                }
            }
        }
    }
}


//------------ MethodNotAllowed ----------------------------------------------

/// Method Not Allowed error.
pub struct MethodNotAllowed<'a> {
    site: &'a Site,
    method: &'a Method,
    path: &'a str,
}

impl<'a> MethodNotAllowed<'a> {
    pub fn new(site: &'a Site, method: &'a Method, path: &'a str) -> Self {
        MethodNotAllowed { site, method, path }
    }
}

impl<'a> Page for MethodNotAllowed<'a> {
    type Nav = nav::Other;

    fn site(&self) -> &Site { self.site }

    fn title(&self, tmpl: &mut TemplateBuffer) {
        if self.site.lang() == Lang::De {
            tmpl.write_str("405. Methode nicht erlaubt");
        }
        else {
            tmpl.write_str("405. Method Not Allowed");
        }
    }

    fn header(&self, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            h1 {
                @ if self.site.lang() == Lang::De {
                    : "405 Methode nicht zugelassen"
                }
                else {
                    : "405 Method Not Allowed"
                }
            }
        }
    }

    fn content(&self, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            p {
                @ if self.site.lang() == Lang::De {
                    : "Die Methode ";
                    tt { : self.method.as_str() }
                    : " ist für den Pfad ";
                    tt { : self.path }
                    : " nicht erlaubt."
                }
                else {
                    : "The method ";
                    tt { : self.method.as_str() }
                    : " is not allowed at ";
                    tt { : self.path }
                    : "."
                }
            }
        }
    }
}

