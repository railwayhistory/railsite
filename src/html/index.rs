/// Index pages.

use crate::site::Site;
use super::{i18n, skeleton};
use super::target::{Content, Target, empty};


//------------ home ----------------------------------------------------------

/// Renders the home page.
pub fn home(site: &Site) -> Target {
    skeleton::styled(
        site, i18n::index::term_home_title,
        empty,
        |cont: &mut Content| {
            cont.element("header").id("frame-home-header").content(|cont| {
                cont.div().class("home-header-bar").content(|cont| {
                    cont.div().class("home-header").content(|cont| {
                        home_header_lang(site, cont);
                        home_header_brand(site, cont);
                    })
                })
            });
            cont.div().id("frame-core").content(|cont| {
                cont.element("main").class("core-content").content(|cont| {
                    cont.element("nav").class("home-nav").content(|cont| {
                        home_nav_content(site, cont);
                    });
                    cont.div().class("home-search").content(|cont| {
                        home_search_content(site, cont);
                    });
                    cont.div().class("home-info").content(|cont| {
                        cont.div().class("home-info-item").content(|cont| {
                            home_country_lines(site, cont);
                        });
                        cont.div().class("home-info-item").touch();
                        cont.div().class("home-info-item").touch();
                        cont.div().class("home-info-item").content(|cont| {
                            home_statistics(site, cont);
                        });
                    });
                });
            });
        },
        |cont: &mut Content| {
            cont.linked_script(site.link_home_js())
        }
    )
}

fn home_header_lang(site: &Site, cont: &mut Content) {
    cont.div().class("frame-nav").content(|cont| {
        skeleton::frame_nav_lang(site, cont);
    })
}

fn home_header_brand(site: &Site, cont: &mut Content) {
    cont.div().class("home-brand").content(|cont| {
        cont.img()
            .src(site.link_brand_logo())
            .alt(i18n::index::term_home_title)
        ;
        cont.p().text(i18n::index::term_home_title);
    });
}

fn home_nav_content(_site: &Site, cont: &mut Content) {
    cont.ul().content(|cont| {
        cont.li().content(|cont| {
            cont.a().href("#").text(i18n::nav::term_browse)
        });
        cont.li().content(|cont| {
            cont.a().href("#").text(i18n::nav::term_map)
        });
        cont.li().content(|cont| {
            cont.a().href("#").text(i18n::nav::term_manual)
        });
    })
}

fn home_search_content(site: &Site, cont: &mut Content) {
    cont.element("form")
        .attr("action", site.link("search"))
        .attr("method", "get")
        .class("home-search-input")
    .content(|cont| {
        cont.div().class("here-prepend").content(|cont| {
            cont.span().content(|cont| {
                cont.i().class("here-icon").touch()
            })
        });
        cont.element("input")
            .id("home-search-input")
            .attr("type", "text")
            .attr(
                "placeholder", i18n::nav::term_search_placeholder
            )
            .attr("aria-label", i18n::nav::term_search)
            .attr("name", "q")
            .attr("autocomplete", "off")
        ;
        cont.div().class("here-append").content(|cont| {
            cont.element("button").attr("type", "submit")
            .text(i18n::nav::term_search)
        });
    });
    cont.div().id("home-search-result").touch();
}

fn home_country_lines(site: &Site, cont: &mut Content) {
    cont.div().class("here-header").text(
        i18n::index::term_home_country_lines
    );
    cont.ul().class("here-body here-list").content(|cont| {
        for link in site.catalogue().countries.iter(cont.lang()) {
            let item = link.follow(site.library());
            cont.li().content(|cont| {
                cont.a().href(
                    site.link_organization_lines(item)
                ).text(
                    item.local_short_name(site.lang().into())
                );
                cont.text(format_args!(" ({})",
                    site.catalogue().country_lines.by_link(link).len()
                ));
            })
        }
    });
}

fn home_statistics(site: &Site, cont: &mut Content) {
    cont.div().class("here-header").text(
        i18n::index::term_home_statistics
    );
    cont.ul().class("here-body here-list").content(|cont| {
        let nums = site.catalogue().doc_nums;
        cont.li().content(|cont| {
            i18n::index::home_statistics_total_documents(
                site.lang(), nums.total, cont
            )
        });
        cont.li().content(|cont| cont.ul().content(|cont| {
            cont.li().content(|cont| {
                i18n::index::home_statistics_line_num(
                    site.lang(), nums.lines, cont
                )
            });
            cont.li().content(|cont| {
                i18n::index::home_statistics_point_num(
                    site.lang(), nums.points, cont
                )
            });
            cont.li().content(|cont| {
                i18n::index::home_statistics_structure_num(
                    site.lang(), nums.structures, cont
                )
            });
            cont.li().content(|cont| {
                i18n::index::home_statistics_organization_num(
                    site.lang(), nums.organizations, cont
                )
            });
            cont.li().content(|cont| {
                i18n::index::home_statistics_source_num(
                    site.lang(), nums.sources, cont
                )
            });
        }));
        /*
        for item in site.catalogue().doc_nums() {
            cont.li().content(|cont| {
                cont.text(item.1);
                cont.text(" ");
                cont.text(item.0);
                cont.text("s");
            })
        }
        */
    });
}
