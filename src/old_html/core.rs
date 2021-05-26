//! The core of our HTML output.
//!
use horrorshow::{html, owned_html, Raw, Template, TemplateBuffer};
use horrorshow::helper::doctype;
use hyper::StatusCode;
use hyper::http::response::Builder as ResponseBuilder;
use crate::http::Response;
use crate::i18n::Lang;
use crate::site::Site;
use super::nav::Nav;


//------------ Html ----------------------------------------------------------

/// Any kind of rendered HTML.
pub trait Html {
    /// Returns a reference to the site.
    fn site(&self) -> &Site;

    /// Renders the HTML content into a string.
    fn to_string(&self) -> String;

    /// Creates a response with rendered HTML and the given status code.
    fn response(&self, status: StatusCode) -> Response {
        ResponseBuilder::new()
            .status(status)
            .header("Content-Type", "text/html;charset=utf-8")
            .header("Set-Cookie", self.site().lang().cookie())
            .body(self.to_string().into())
            .unwrap()
    }

    /// Creates a success response with the rendered HTML.
    fn ok(&self) -> Response {
        self.response(StatusCode::OK)
    }
}


//------------ Shell ---------------------------------------------------------

/// The basic, bare shell of a page.
///
/// This builds the HTML structure and pulls in the style sheets and standard
/// java script we always need.
pub trait Shell {
    /// Returns a reference to the site.
    fn site(&self) -> &Site;

    /// Renders the page title.
    fn title(&self, tmpl: &mut TemplateBuffer) {
        tmpl.write_raw("railwayhistory.org");
    }

    /// Renders the page body.
    fn body(&self, _tmpl: &mut TemplateBuffer) { }

    /// Renders additional head content.
    fn head(&self, _tmpl: &mut TemplateBuffer) { }

    /// Renders additional scripts.
    fn scripts(&self, _tmpl: &mut TemplateBuffer) { }
}

impl<T: Shell> Html for T {
    fn site(&self) -> &Site {
        <T as Shell>::site(self)
    }

    fn to_string(&self) -> String {
        (html! {
            : doctype::HTML;
            html(lang=self.site().lang().code()) {
                head {
                    title {
                        |tmpl| self.title(tmpl)
                    }
                    meta(
                        name = "viewport",
                        value = "width=device-width, initial-scale=1, \
                                 shrink-to-fit=no"
                    );
                    link(
                        rel = "stylesheet",
                        href = self.site().link("static/style.css")
                    );
                    |tmpl| { self.head(tmpl); }
                }
                body {
                    |tmpl| { self.body(tmpl); }
                    script(
                        src = self.site().link("static/js/jquery.min.js")
                    ) { }
                    script(
                        src = self.site().link("static/js/popper.min.js")
                    ) { }
                    script(
                        src = self.site().link("static/js/bootstrap.min.js")
                    ) { }
                    |tmpl| { self.scripts(tmpl); }
                }
            }
        }).into_string().unwrap()
    }
}


//------------ Page ----------------------------------------------------------

/// A standard page of the site.
///
/// This trait is used as the basis for pretty much all pages of the site.
pub trait Page {
    /// The navigation variant to use for this page.
    type Nav: Nav;

    /// Returns a reference to the site.
    fn site(&self) -> &Site;

    /// Renders the page title.
    fn title(&self, tmpl: &mut TemplateBuffer) {
        tmpl.write_raw("railwayhistory.org");
    }

    /// Renders the head line of the page.
    fn header(&self, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            header {
                h1 { |tmpl| self.title(tmpl); }
            }
        }
    }

    /// Renders the content of the page.
    fn content(&self, _tmpl: &mut TemplateBuffer) { }

    /// Renders the footer content.
    fn footer(&self, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            p {
                @ if self.site().lang() == Lang::De {
                    : "Die Railway History Database ist ";
                    : "unter der ";
                    a(href="https://creativecommons.org/licenses/by/4.0/") {
                        : "Creative Commons Attribution ";
                        : "4.0 International"
                    }
                    : ", Geodaten unter der ";
                    a(href="https://opendatacommons.org/licenses/odbl/") {
                        : "Open Data Commons Open Database ";
                        : "License (ODbL)";
                    }
                    : " lizensiert. Bilder und andere ";
                    : "Medien können abweichend lizensiert ";
                    : "sein.";
                } else {
                    : "The Railway History Database is ";
                    : "made available under the ";
                    a(href="https://creativecommons.org/licenses/by/4.0/") {
                        : "Creative Commons Attribution ";
                        : "4.0 International";
                    }
                    : " license except for geographical ";
                    : "data which is published under the ";
                    a(href="https://opendatacommons.org/licenses/odbl/") {
                        : "Open Data Commons Open Database ";
                        : "License (ODbL)"
                    }
                    : ". Images and other artwork may be ";
                    : "licensed differently."
                }
            }
        }
    }

    /// Renders additional head content.
    fn head(&self, _tmpl: &mut TemplateBuffer) { }

    /// Renders additional scripts.
    fn scripts(&self, _tmpl: &mut TemplateBuffer) { }
}

impl<T: Page> Shell for T {
    fn site(&self) -> &Site {
        <T as Page>::site(self)
    }

    fn title(&self, tmpl: &mut TemplateBuffer) {
        <T as Page>::title(self, tmpl)
    }

    fn body(&self, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            div(id = "frame") {
                nav(id = "frame-header") {
                    div(class = "frame-nav-bar") {
                        div(class = "frame-nav") {
                            a(
                                class = "frame-nav-brand",
                                href = self.site().link("")
                            ) {
                                img(
                                    src = self.site().link(
                                        "static/img/logo64-beige.svg"
                                    ),
                                    width = 64,
                                    alt = "railwayhistory.org"
                                );
                            }
                            button(
                                class = "navbar-toggler",
                                type = "button",
                                data-toggle = "collapse",
                                data-target = "#frame-nav-content",
                                aria-controls = "frame-nav-content",
                                aria-expanded = "false",
                                aria-label = "Toggle navigation bar"
                            ) {
                                span(class = "navbar-toggler-icon") { }
                            }

                            div(
                                class = "frame-nav-content collapse",
                                id="frame-nav-content"
                            ) {
                                ul(class = "frame-nav-chapters") {
                                    li {
                                        a(href = self.site().link("")) {
                                            : term_home(self.site())
                                        }
                                    }
                                    li {
                                        a(href = "#") {
                                            : term_browse(self.site())
                                        }
                                    }
                                    li {
                                        a(href = "#") {
                                            : term_map(self.site())
                                        }
                                    }
                                    li {
                                        a(href = self.site().link_docs()) {
                                            : term_documentation(self.site())
                                        }
                                    }
                                }
                                : frame_nav_search(self.site());
                                : frame_nav_lang(self.site())
                            }
                        }
                    }
                }
                div(id = "frame-core") {
                    div(class = "core-header-bar") {
                        div(class = "core-header") {
                            |tmpl| { self.header(tmpl); }
                        }
                    }
                    div(class = "core-content-bar") {
                        main(class = "core-content") {
                            |tmpl| { self.content(tmpl); }
                        }
                    }
                }
                footer(id = "frame-footer") {
                    div(class = "footer-content") {
                        |tmpl| { self.footer(tmpl); }
                    }
                }
            }
        }
    }

    fn head(&self, tmpl: &mut TemplateBuffer) {
        <T as Page>::head(self, tmpl)
    }

    fn scripts(&self, tmpl: &mut TemplateBuffer) {
        <T as Page>::scripts(self, tmpl);
        tmpl << html! {
            script {
                : Raw(include_str!("../../js/inc/page.js"))
            }
        };
    }
}

// Helper Functions.

fn frame_nav_search<'a>(site: &'a Site) -> impl Template + 'a {
    owned_html! {
        div(class = "frame-nav-search") {
            form(
                action = site.link("search"),
                method = "get",
                id = "foo"
            ) {
                input(
                    class = "dropdown",
                    id = "frame-nav-search-input",
                    type="text",
                    placeholder = term_search(site),
                    aria-label = term_search(site),
                    name = "q",
                    autocomplete = "off"
                );
                div {
                    button(type = "submit") {
                        i(class="frame-icon-search") { }
                    }
                }
            }
            div(
                class = "frame-nav-search-result",
                id = "frame-nav-search-result"
            ) {
            }
        }
    }
}

fn frame_nav_lang<'a>(site: &'a Site) -> impl Template + 'a {
    owned_html! {
        ul(class = "frame-nav-lang") {
            li {
                a(
                    href = "#",
                    role="button",
                    data-toggle="dropdown",
                    aria-haspopup="true",
                    aria-expanded="false",
                    id="navbarDropdown"
                ) {
                    i(class = "frame-icon-lang") { }
                }
                div(
                    class = "frame-nav-lang-menu dropdown-menu dropdown-menu-right",
                    aria-labelledby="navbarDropdown"
                ) {
                    a (
                        href = "?lang=en",
                        class = if site.lang() == Lang::En {
                            "active"
                        } else {
                            ""
                        }
                    ) {
                        : "English"
                    }
                    a (
                        href = "?lang=de",
                        class = if site.lang() == Lang::De {
                            "active"
                        } else {
                            ""
                        }
                    ) {
                        : "Deutsch"
                    }
                }
            }
        }
    }
}


//------------ I18N ----------------------------------------------------------

fn term_search(site: &Site) -> &'static str {
    match site.lang() {
        Lang::De => "Suchen",
        Lang::En => "Search",
    }
}

fn term_home(site: &Site) -> &'static str {
    match site.lang() {
        Lang::De => "Start",
        Lang::En => "Home",
    }
}

fn term_browse(site: &Site) -> &'static str {
    match site.lang() {
        Lang::De => "Übersichten",
        Lang::En => "Browse",
    }
}

fn term_map(site: &Site) -> &'static str {
    match site.lang() {
        Lang::De => "Karte",
        Lang::En => "Map",
    }
}

fn term_documentation(site: &Site) -> &'static str {
    match site.lang() {
        Lang::De => "Dokumentation",
        Lang::En => "Documentation",
    }
}


