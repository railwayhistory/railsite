//! The core of our HTML output.
//!
use horrorshow::{html, RenderOnce, Template, TemplateBuffer};
use horrorshow::helper::doctype;
use crate::http::Request;
use crate::i18n::Lang;
use crate::site::{home, statics};


//------------ Functions -----------------------------------------------------

pub fn other(
    request: &Request,
    title: impl RenderOnce,
    body: impl RenderOnce
) -> String {
    core(request, title, "", Nav::Other, "", body, "")
}

pub fn core(
    request: &Request,
    title: impl RenderOnce,
    head: impl RenderOnce,
    nav: Nav,
    core_header: impl RenderOnce,
    content: impl RenderOnce,
    scripts: impl RenderOnce,
) -> String {
    (html! {
        : doctype::HTML;
        html(lang=request.lang().code()) {
            head {
                title :title;
                meta(charset="utf-8");
                meta(
                    name="viewport",
                    value="width=device-width, initial-scale=1, \
                           shrink-to-fit=no"
                );
                link(rel="stylesheet", href=statics::style_css(request));
                : head;
            }
            body {
                div(id="frame") {
                    header(id="frame-header") {
                        div(class="brand-bar") {
                            div(class="brand") {
                                a(href=home::home(request)) {
                                    :"The Railway History Database"
                                }
                            }
                        }
                        nav(class="frame-nav-bar") {
                            div(class="frame-nav") {
                                |tmpl| nav.render_once(request, tmpl);
                            }
                        }
                    }
                    div(id="frame-core") {
                        div(class="core-header-bar") {
                            header(class="core-header") :core_header;
                        }
                        div(class="core-content") :content;
                    }
                    footer(id="frame-footer") {
                        div(class="footer-content") {
                            p {
                                @ if request.lang() == Lang::De {
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
                }
                script(src=statics::jquery_js(request)) { }
                script(src=statics::popper_js(request)) { }
                script(src=statics::bootstrap_js(request)) { }
                : scripts
            }
        }
    }).into_string().unwrap()
}

pub enum Nav {
    Other
}

impl Nav {
    fn render_once(self, request: &Request, tmpl: &mut TemplateBuffer) {
        match self {
            Nav::Other => {
                tmpl << html! {
                    ul(class="frame-nav-parts") {
                        li {
                            a(href=home::home(request)) {
                                span(class="icon", title="Home") {
                                    i(class="frame-icon-home") { }
                                }
                                span(class="text") {
                                    @ if request.lang() == Lang::De {
                                        : "Start"
                                    } else {
                                        : "Home"
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

