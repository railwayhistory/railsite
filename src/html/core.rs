//! The core of our HTML output.
//!
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use actix_web::http::StatusCode;
use htmlfn::core::Content;


//------------ Functions -----------------------------------------------------

pub fn other<T: Content, B: Content>(
    lang: &'static str,
    title: T,
    body: B
) -> HtmlContent {
    core(lang, title, Nav::Other, body)
}


pub fn core<T: Content, B: Content>(
    lang: &'static str,
    title: T,
    nav: Nav,
    body: B
) -> HtmlContent {
    HtmlContent::ok(html!(
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
    ))
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


//------------ HtmlContent ---------------------------------------------------

pub struct HtmlContent {
    status: StatusCode,
    content: String,
}

impl HtmlContent {
    pub fn ok<C: Content>(content: C) -> Self {
        HtmlContent {
            status: StatusCode::OK,
            content: content.into_string(),
        }
    }

    pub fn status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }
}

impl Responder for HtmlContent {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(
        self,
        req: &HttpRequest<S>
    ) -> Result<HttpResponse, Error> {
        Ok(req
            .build_response(self.status)
            .content_type("text/html; charset=utf-8")
            .body(self.content)
        )
    }
}

