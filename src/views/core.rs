//! The core of our HTML output.
//!
use htmlfn::core::Content;

pub fn other<T: Content, B: Content>(
    lang: &'static str,
    title: T,
    body: B
) -> impl Content {
    core(lang, title, Nav::Other, body)
}


pub fn core<T: Content, B: Content>(
    lang: &'static str,
    title: T,
    nav: Nav,
    body: B
) -> impl Content {
    html!(
        html(lang=lang) {
            head {
                title { title }
                meta(charset="utf-8") { }
                meta(
                    name="viewport",
                    value="width=device-width, initial-scale=1, \
                           shrink-to-fit=no",
                ) { }
                link(rel="stylesheet", href="/static/style.css") { }
            }
            body {
                nav(class="navbar navbar-expand-md navbar-light bg-light") {
                    nav.content()
                }
                main(role="main", class="container") {
                    body
                }
                script(src="/static/js/jquery.min.js") { }
                script(src="/static/js/popper.min.js") { }
                script(src="/static/js/bootstrap.min.js") { }
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
            a(class="navbar-brand", href="/") {
                "RWH"
            }
            button(
                class="navbar-toggler",
                "type" => "button",
                "data-toggle" => "collaps",
                "data-target" => "#navbarDefault"
            ) {
                span(class="navbar-toggler-icon") { }
            }
            div(class="collapse navbar-collapse", id="navbarDefault") {
                ul(class="navbar-nav mr-auto") {
                    li(class="nav-item") {
                        a(class="nav-link", href="#") {
                            "First"
                        }
                    }
                    li(class="nav-item") {
                        a(class="nav-link", href="#") {
                            "Second"
                        }
                    }
                    li(class="nav-item") {
                        a(class="nav-link", href="#") {
                            "Third"
                        }
                    }
                }
            }
        )
    }
}

