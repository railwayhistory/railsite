//! The skeletons of HTML documents for the RWH site.

use crate::i18n::Lang;
use crate::site::Site;
use super::i18n;
use super::target::{Content, RenderText, Target};


//------------ Public Functions ----------------------------------------------

/// The most basic of HTML documents.
///
/// All other skeletons are based off of this one.
///
/// The language of the text in the document is given via `lang`. The
/// content of the title element is constructed via `title`. All other
/// elements of the header are constructed via `head`. The regular body
/// content is constructed via `body` with any scripts placed at the end
/// of the body via `scripts`.
pub fn base(
    lang: Lang,
    title: impl RenderText,
    head: impl FnOnce(&mut Content),
    body: impl FnOnce(&mut Content),
    scripts: impl FnOnce(&mut Content),
) -> Target {
    Target::new(lang).html(|cont: &mut Content| {
        cont.element("head").content(|cont| {
            cont.element("title").text(title);
            head(cont);
        });
        cont.element("body").content(|cont| {
            body(cont);
            scripts(cont);
        });
    })
}


/// A basic page with all our basic styling included.
pub fn styled(
    site: &Site,
    title: impl RenderText,
    head: impl FnOnce(&mut Content),
    body: impl FnOnce(&mut Content),
    scripts: impl FnOnce(&mut Content),
) -> Target {
    base(
        site.lang(),
        title,
        |cont: &mut Content| { // head
            cont.element("meta")
                .attr("name", "viewport")
                .attr(
                    "content",
                    "width=device-width, initial-scale=1.0, shrink-to-fit=no"
                )
            ;
            cont.element("link")
                .attr("rel", "stylesheet")
                .href(site.link_style_css())
            ;
            head(cont);
        },
        body,
        |cont: &mut Content| {
            basic_scripts(site, cont);
            scripts(cont);
        }
    )
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Nav {
    Home,
    Browse,
    Map,
    Documentation,
    Other
}


/// A standard page.
pub fn standard(
    site: &Site,
    title: impl RenderText,
    head: impl FnOnce(&mut Content),
    nav: Nav,
    core: impl FnOnce(&mut Content),
    scripts: impl FnOnce(&mut Content),
) -> Target {
    styled(
        site, title, head,
        |cont: &mut Content| { // body
            cont.element("nav").id("frame-header").content(|cont| {
                cont.div().class("frame-nav-bar").content(|cont| {
                    cont.div().class("frame-nav").content(|cont| {
                        frame_nav_content(nav, site, cont)
                    })
                })
            });
            cont.div().id("frame-core").content(core);
            cont.element("footer").id("frame-footer").content(|cont| {
                cont.div().class("footer-content").content(|cont| {
                    footer_content(site, cont)
                })
            })
        },
        scripts
    )
}


/// A standard page with a headline.
pub fn headline(
    site: &Site,
    title: impl RenderText,
    head: impl FnOnce(&mut Content),
    nav: Nav,
    headline: impl FnOnce(&mut Content),
    content: impl FnOnce(&mut Content),
    scripts: impl FnOnce(&mut Content),
) -> Target {
    standard(
        site, title, head, nav,
        |cont: &mut Content| {
            cont.div().class("core-header-bar").content(|cont| {
                cont.div().class("core-header").content(headline)
            });
            cont.div().class("core-content-bar").content(|cont| {
                cont.div().class("core-content").content(content)
            });
        },
        scripts
    )
}


/// A standard page with a headline and content parts
pub fn headline_parts(
    site: &Site,
    title: impl RenderText,
    head: impl FnOnce(&mut Content),
    nav: Nav,
    headline: impl FnOnce(&mut Content),
    content_nav: impl FnOnce(&mut Content),
    content: impl FnOnce(&mut Content),
    scripts: impl FnOnce(&mut Content),
) -> Target {
    standard(
        site, title, head, nav,
        |cont: &mut Content| {
            cont.div().class("core-header-bar").content(|cont| {
                cont.div().class("core-header").content(headline)
            });
            cont.div().class("core-nav-bar").content(|cont| {
                cont.div().class("core-nav").content(|cont| {
                    cont.ul().content(content_nav)
                });
            });
            cont.div().class("core-content-bar").content(|cont| {
                cont.div().class("core-content").content(content)
            });
        },
        scripts
    )
}


//---------- sheet -----------------------------------------------------------

/// Sheets are the basic content pages consisting of a headline and content.
pub fn sheet(
    site: &Site,
    title: impl RenderText,
    head: impl FnOnce(&mut Content),
    nav: Nav,
    headline: impl FnOnce(&mut Content),
    content: impl FnOnce(&mut Content),
    scripts: impl FnOnce(&mut Content),
) -> Target {
    standard(
        site, title, head, nav,
        |cont: &mut Content| {
            cont.div().class("core-header-bar").content(|cont| {
                cont.div().class("core-header").content(headline)
            });
            cont.div().class("core-content-bar").content(|cont| {
                cont.div().class("core-content").content(content)
            });
        },
        scripts
    )
}


//----------- multipart_sheet ------------------------------------------------

/// A sheet with the content split into multiple parts.
///
/// Instead of the content of a regular sheet, a multipart sheet has a _part
/// nav_ that allows to quickly navigate to individual parts followed by a
/// series of parts.
pub fn multipart_sheet(
    site: &Site,
    title: impl RenderText,
    head: impl FnOnce(&mut Content),
    nav: Nav,
    headline: impl FnOnce(&mut Content),
    part_nav: impl FnOnce(&mut PartNav),
    part_panel: impl FnOnce(&mut Content),
    scripts: impl FnOnce(&mut Content)
) -> Target {
    standard(
        site, title, head, nav,
        |cont: &mut Content| {
            cont.div().class("core-header-bar").content(|cont| {
                cont.div().class("core-header").content(headline)
            });
            cont.div().class("core-nav-bar").content(|cont| {
                cont.div().class("core-nav").content(|cont| {
                    let mut ul = cont.ul();
                    part_nav(&mut PartNav::new(ul.get_content()));
                });
            });
            cont.div().class("core-content-bar").content(|cont| {
                cont.div().class("core-content").content(|cont| {
                    part_panel(cont)
                })
            });
        },
        scripts
    )
}

pub struct PartNav<'a> {
    content: Content<'a>,
}

impl<'a> PartNav<'a> {
    fn new(content: Content<'a>) -> Self {
        PartNav { content }
    }

    pub fn item(
        &mut self,
        href: impl RenderText,
        active: bool,
        content: impl FnOnce(&mut Content)
    ) {
        let class = if active {
            "active"
        }
        else { 
            ""
        };
        self.content.li().content(|cont| {
            cont.a()
                .href(href)
                .class(class)
            .content(content)
        })
    }
}


//------------ Private Helpers -----------------------------------------------

fn frame_nav_content(nav: Nav, site: &Site, cont: &mut Content) {
    cont.a().class("frame-nav-brand").href(site.link_home())
    .content(|cont| {
        cont.img()
            .attr("src", site.link_brand_logo())
            .attr("width", "64")
            .attr("alt", i18n::nav::term_home);
    });
    cont.element("button")
        .class("navbar-toggler")
        .attr("type", "button")
        .attr("data-toggle", "collapse")
        .attr("data-target", "#frame-nav-content")
        .attr("aria-controls", "frame-nav-content")
        .attr("aria-expanded", "false")
        .attr("aria-label", "Toggle navigation bar")
    .content(|cont| {
        cont.span().class("navbar-toggler-icon").touch();
    });
    cont.div().class("frame-nav-content collapse").id("frame-nav-content")
        .content(|cont| {
            cont.ul().class("frame-nav-chapters").content(|cont| {
                let mut li = cont.li();
                if nav == Nav::Home {
                    li = li.class("active")
                }
                li.content(|cont| {
                    cont.a().href(site.link_home()).text(i18n::nav::term_home)
                });

                let mut li = cont.li();
                if nav == Nav::Browse {
                    li = li.class("active")
                }
                li.content(|cont| {
                    cont.a().href(site.link("#")).text(i18n::nav::term_browse)
                });

                let mut li = cont.li();
                if nav == Nav::Map {
                    li = li.class("active")
                }
                li.content(|cont| {
                    cont.a().href(site.link("#")).text(i18n::nav::term_map)
                });

                let mut li = cont.li();
                if nav == Nav::Documentation {
                    li = li.class("active")
                }
                li.content(|cont| {
                    cont.a().href(site.link("#")).text(i18n::nav::term_manual)
                });
            });
            frame_nav_search(site, cont);
            frame_nav_lang(site, cont);
        });
}

fn frame_nav_search(site: &Site, cont: &mut Content) {
    cont.div().class("frame-nav-search").content(|cont| {
        cont.element("form")
            .attr("action", site.link("search"))
            .attr("method", "get")
        .content(|cont| {
            cont.element("input")
                .class("dropdown")
                .id("frame-nav-search-input")
                .attr("type", "text")
                .attr(
                    "placeholder", i18n::nav::term_search
                )
                .attr(
                    "aria-label", i18n::nav::term_search
                )
                .attr("name", "q")
                .attr("autocomplete", "off")
            ;
            cont.div().content(|cont| {
                cont.element("button").attr("type", "submit")
                .content(|cont| {
                    cont.i().class("frame-icon-search").touch();
                })
            });
        });
        cont.div()
            .class("frame-nav-search-result")
            .id("frame-nav-search-result")
        .touch();
    })
}

pub fn frame_nav_lang(site: &Site, cont: &mut Content) {
    cont.ul().class("frame-nav-lang").content(|cont| {
        let mut li = cont.li();
        if site.lang() == Lang::En {
            li = li.class("active")
        }
        li.content(|cont| {
            cont.a()
                .href("?lang=en")
                .attr("title", "English")
            .text("EN")
        });

        let mut li = cont.li();
        if site.lang() == Lang::De {
            li = li.class("active")
        }
        li.content(|cont| {
            cont.a()
                .href("?lang=de")
                .attr("title", "Deutsch")
            .text("DE")
        });

        /*
        cont.li().content(|cont| {
            cont.a()
                .href("#")
                .attr("role", "button")
                .attr("data-toggle", "dropdown")
                .attr("aria-haspopup", "true")
                .attr("aria-expanded", "false")
                .id("navbarDropdown")
                .content(|cont| {
                    cont.i().class("frame-icon-lang").touch();
                })
            ;
            cont.div()
                .class("frame-nav-lang-menu dropdown-menu dropdown-menu-right")
                .attr("aria-labelledby", "navbarDropdown")
                .content(|cont| {
                    let mut a = cont.a().href("?lang=en");
                    if site.lang() == Lang::En {
                        a = a.class("active")
                    }
                    a.text("English");

                    let mut a = cont.a().href("?lang=de");
                    if site.lang() == Lang::De {
                        a = a.class("active")
                    }
                    a.text("Deutsch");
                })
            ;
        });
        */
    })
}

fn footer_content(site: &Site, cont: &mut Content) {
    cont.p().content(|cont| i18n::footer::footer_content(site.lang(), cont))
}

fn basic_scripts(site: &Site, cont: &mut Content) {
    cont.linked_script(site.link_jquery_js());
    cont.linked_script(site.link_popper_js());
    cont.linked_script(site.link_bootstrap_js());
    cont.linked_script(site.link_skeleton_js());
}
