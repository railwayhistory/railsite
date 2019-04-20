//! The core of our HTML output.
//!
use htmlfn::core::Content;


//------------ Functions -----------------------------------------------------

pub fn other<T: Content, B: Content>(
    lang: &'static str,
    title: T,
    body: B
) -> impl Content {
    core(lang, title, (), Nav::Other, (), body, ())
}


pub fn core<T: Content, H: Content, CH: Content, C: Content, S: Content>(
    lang: &'static str,
    title: T,
    head: H,
    nav: Nav,
    core_header: CH,
    content: C,
    scripts: S,
) -> impl Content {
    html!(
        html(lang=lang) {
            head() {
                title() { title }
                meta(charset="utf-8");
                meta(
                    name="viewport",
                    value="width=device-width, initial-scale=1, \
                           shrink-to-fit=no",
                );
                link(rel="stylesheet", href="/static/style.css");
                head
            }
            body() {
                div(id="frame") {
                    header(id="frame-header") {
                        div(class="brand-bar") {
                            div(class="brand") {
                                a(href="/") {
                                    "The Railway History Database"
                                }
                            }
                        }
                        nav(class="frame-nav-bar") {
                            div(class="frame-nav") {
                                nav.content()
                            }
                        }
                    }
                    div(id="frame-core") {
                        div(class="core-header-bar") {
                            header(class="core-header") {
                                core_header
                            }
                        }
                        div(class="core-content") {
                            content
                        }
                    }
                    footer(id="frame-footer") {
                        div(class="footer-content") {
                            p() {
                                "The Railway History Database is made \
                                available under the Open Database License. \
                                Any rights in individual contents of the \
                                database are licensed under the Database \
                                Content License. Documentation and other \
                                text content is made available under a \
                                Creative Commons Attribute-Share Alike 3.0 \
                                Unported license. Images and other art work \
                                may be licensed differently."
                            }
                            p() {
                            }
                        }
                    }
                }
                script(src="/static/js/jquery.min.js") { }
                script(src="/static/js/popper.min.js") { }
                script(src="/static/js/bootstrap.min.js") { }
                scripts
            }
        }
    )
}

pub enum Nav {
    Other
}

impl Nav {
    fn content(&self) -> impl Content {
        elements!(
            ul(class="frame-nav-parts") {
                li() {
                    a(href="/") {
                        span(class="icon", title="Home") {
                            i(class="frame-icon-home") { }
                        }
                        span(class="text") { "Home" }
                    }
                }
            }
        )
    }
}

