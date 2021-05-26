use horrorshow::{html, Raw, TemplateBuffer};
use raildata::document::{Document, Line, Organization};
use crate::http::Request;
use crate::i18n::Lang;
use crate::site::Site;
use crate::tools::paginator;
use super::core::{Page, Shell};
use super::nav;


//------------ Home ----------------------------------------------------------

/// The home page.
pub struct Home<'a> {
    site: &'a Site,
}

impl<'a> Home<'a> {
    pub fn new(site: &'a Site) -> Self {
        Home { site }
    }

    fn countries(&'a self) -> impl Iterator<Item = &'a Organization> + 'a {
        self.site.catalogue().countries().map(move |link| {
            link.follow(self.site.library())
        })
    }
}

impl<'a> Shell for Home<'a> {
    fn site(&self) -> &Site { self.site }

    fn title(&self, tmpl: &mut TemplateBuffer) {
        tmpl.write_str("The Railway History Database");
    }

    fn body(&self, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            div(id = "frame") {
                header(id = "frame-home-header") {
                    div(class = "home-header-bar") {
                        div(class = "home-header") {
                            div(class = "home-brand") {
                                img(
                                    src = self.site().link(
                                        "static/img/logo64-beige.svg"
                                    ),
                                    alt = "railwayhistory.org"
                                );
                                p {
                                    : "The Railway History Database";
                                }
                            }
                        }
                    }
                }
                div(id = "frame-core") {
                    main(class = "core-content") {
                        nav(class = "home-nav") {
                            ul {
                                li {
                                    a(href = "#") {
                                        : "Browse"
                                    }
                                }
                                li {
                                    a(href = "#") {
                                        : "Map"
                                    }
                                }
                                li {
                                    a(href = self.site().link_docs()) {
                                        : "Documentation"
                                    }
                                }
                            }
                        }

                        div(class = "home-search") {
                            form(
                                action = self.site().link("search"),
                                method = "get",
                                class = "home-search-input"
                            ) {
                                div(class="here-prepend") {
                                    span {
                                        i(class="here-icon") { }
                                    }
                                }
                                input(
                                    id = "home-search-input",
                                    type = "text",
                                    placeholder = "Search …",
                                    aria-label = "Search",
                                    name = "q"
                                );
                                div(class="here-append") {
                                    button(type = "submit") { : "Search" }
                                }
                            }
                            div(id = "home-search-result") {
                            }
                        }

                        div(class = "home-info") {
                            div(class = "home-info-item") {
                                div(class = "here-header") {
                                    : "Lines by Country"
                                }
                                div(class = "here-body here-list") {
                                    @ for item in self.countries() {
                                        li {
                                            a(
                                                href =
                                                    self.site.link_country_lines(item)
                                            ) {
                                                : item.local_name(self.site.lang().into())
                                            }
                                        }
                                    }
                                }
                            }
                            div(class = "home-info-item") {
                            }
                            div(class = "home-info-item") {
                            }
                            div(class = "home-info-item") {
                                div(class = "here-header") {
                                    : "Statistics"
                                }
                                div(class = "here-body here-list") {
                                    @ for item in self.site()
                                                      .catalogue().doc_nums()
                                    {
                                        li {
                                            : item.1;
                                            : " ";
                                            : item.0.to_string();
                                            : "s"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn scripts(&self, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            script {
                : Raw(include_str!("../../js/inc/home.js"))
            }
        }
    }
}


//------------ Lines ---------------------------------------------------------

/// An index of all lines.
pub struct Lines<'a> {
    site: &'a Site,
    page: paginator::Page,
}

impl<'a> Lines<'a> {
    const PAGEINATOR: paginator::Paginator
        = paginator::Paginator::with_orphans(50, 4);

    pub fn new(site: &'a Site, request: &Request) -> Self {
        Lines {
            site,
            page: Self::PAGEINATOR.into_page(request, Self::line_num(site)),
        }
    }

    fn line_num(site: &Site) -> usize {
        site.library().iter().filter(|item| {
            match *item {
                Document::Line(_) => true,
                _ => false
            }
        }).count()
    }

    fn lines(&self) -> impl Iterator<Item = (&Document, &Line)> {
        self.site.library().iter().filter_map(|item| {
            match *item {
                Document::Line(ref line) => Some((item, line)),
                _ => None
            }
        })
    }
}

impl<'a> Page for Lines<'a> {
    type Nav = nav::Other;

    fn site(&self) -> &Site { self.site }

    fn title(&self, tmpl: &mut TemplateBuffer) {
        tmpl.write_str(match self.site.lang() {
            Lang::De => "Streckenindex",
            Lang::En => "Line Index",
        })
    }

    fn content(&self, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            |tmpl| self.page.render(tmpl);
            table(class = "table") {
                @ for (doc, line) in self.page.iter(self.lines()) {
                    tr {
                        td {
                            a(href = self.site.link_document(doc)) {
                                : self.site.linktext_line(line);
                            }
                        }
                        td {
                            @ for (idx, point) in line.junctions(
                                self.site.library()
                            ).enumerate() {
                                @ if idx > 0 {
                                    : " – "
                                }
                                :point.name(line.jurisdiction())
                            }
                        }
                    }
                }
            }
        }
    }
}

