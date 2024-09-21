use htmlfn::core::{Content, Text};
use htmlfn::html;
use httools::hyper::Body;
use httools::response::{ContentType, Response, ResponseBuilder};
use crate::page::panels;
use crate::route;
use crate::state::RequestState;


//------------ Page ----------------------------------------------------------

/// A trait for any page.
pub trait Page: Into<Body> {
    fn response(self, builder: ResponseBuilder) -> Response {
        builder.content_type(ContentType::HTML).body(self.into())
    }

    fn ok(self, state: &RequestState) -> Response {
        self.response(state.response())
    }
}


//------------ skeleton ------------------------------------------------------

/// The most basic frame.
///
/// The language of the page will be taken from `state`.  The header
/// will contain a title with the given text and additional head elements
/// from `head`. All JavaScript links should go into `scripts`.
/// Finally, the page’s body is taken from `body`.
pub fn skeleton<'a>(
    state: &'a RequestState,
    title: impl Text + 'a,
    head: impl Content + 'a,
    scripts: impl Content + 'a,
    body: impl Content + 'a,
) -> impl Page + 'a {
    Frame(
        (
            html::doctype(),
            html::html(state.lang(),
                html::head((
                    html::title(title),
                    head,
                )),
                html::body((
                    body,
                    scripts
                )),
            )
        )
    )
}

//------------ basic ---------------------------------------------------------

/// A basic page with all styling included.
pub fn basic<'a>(
    state: &'a RequestState,
    title: impl Text + 'a,
    head: impl Content + 'a,
    scripts: impl Content + 'a,
    body: impl Content + 'a,
) -> impl Page + 'a {
    skeleton(
        state,
        title,
        (
            html::meta::utf8(),
            html::meta::viewport(
                "width=device-width, initial-scale=1.0, shrink-to-fit=no"
            ),
            html::link::stylesheet(route::assets::StyleCss::href(state)),
            head
        ),
        (
            //basic_scripts(state),
            scripts,
        ),
        body
    )
}


//------------ standard ------------------------------------------------------

/// A standard content page.
pub fn standard<'a>(
    state: &'a RequestState,
    title: impl Text + 'a,
    head: impl Content + 'a,
    scripts: impl Content + 'a,
    core: impl Content + 'a,
) -> impl Page + 'a {
    basic(
        state, title, head, scripts,
        (
            html::header::id("root-header",
                html::div::class("root-area",
                    panels::header::standard(state)
                )
            ),
            html::main::id("root-main",
                html::div::class("root-area",
                    core
                )
            ),
            html::footer::id("root-footer",
                html::div::class("root-area",
                    panels::footer::standard(state)
                )
            ),
        )
    )
}


//------------ Frame ---------------------------------------------------------

struct Frame<Cont>(Cont);

impl<Cont: Content> Into<Body> for Frame<Cont> {
    fn into(self) -> Body {
        self.0.render().into()
    }
}

impl<Cont: Content> Page for Frame<Cont> { }

