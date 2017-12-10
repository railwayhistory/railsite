//! The core of our HTML output.
//!
use ::htmlfn::core::{AttrValue, Text};
use ::htmlfn::html;

pub fn core<L, T, N, B>(lang: L, title_content: T, nav: N, body: B) -> String
            where L: AttrValue, T: Text, N: Text, B: Text {
    html::html().lang(lang).content((
        html::head().content((
            html::title().content(title_content),
            html::meta().charset("utf-8"),
            html::meta().name("viewport")
                        .value("width=device-width, initial-scale=1, \
                                shrink-to-fit=no"),
            html::link().rel("stylesheet")
                        .href("/static/style.css")
        )),
        html::body().content((
            html::nav().class("nav navbar-expand-md navbar-dark bg-dark \
                              fixed-top")
                       .content(nav),
            html::main().role("main").class("container").content(body),
            html::script().src("/static/js/jquery.min.js").content(()),
            html::script().src("/static/js/popper.min.js").content(()),
            html::script().src("/static/js/bootstrap.min.js").content(()),
        ))
    )).render_string().unwrap()
}
