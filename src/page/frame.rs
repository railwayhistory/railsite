use htmlfn::core::{Content, Text};
use htmlfn::html;
use htmlfn::html::attr;
use httools::hyper::Body;
use httools::response::{ContentType, Response, ResponseBuilder};
use crate::{i18n, route};
use crate::state::RequestState;


//------------ Page ----------------------------------------------------------

/// A trait for any page.
pub trait Page: Into<Body> {
    fn response(self, builder: ResponseBuilder) -> Response {
        builder.content_type(ContentType::HTML).body(self.into())
    }

    fn ok(self) -> Response {
        self.response(ResponseBuilder::new())
    }
}


//------------ skeleton ------------------------------------------------------

/// The most basic frame.
///
/// The language will of the page will be taken from `state`.  The header
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
            html::link::stylesheet(route::assets::StyleCss::link(state)),
            head
        ),
        (
            //basic_scripts(state),
            scripts,
        ),
        body
    )
}

//------------ Nav -----------------------------------------------------------

/// The navigation category a view lives in.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum Nav {
    Home,
    Browse,
    Map,
    Documentation,
    Other
}


//------------ standard ------------------------------------------------------

/// A standard content page.
pub fn standard<'a>(
    state: &'a RequestState,
    title: impl Text + 'a,
    head: impl Content + 'a,
    scripts: impl Content + 'a,
    nav: Nav,
    core: impl Content + 'a,
) -> impl Page + 'a {
    basic(
        state, title, head, scripts,
        (
            html::nav::id("frame-header", 
                html::div::class("frame-nav-bar", 
                    html::div::class("frame-nav", 
                        frame_nav_content(state, nav)
                    )
                )
            ),
            html::div::id("frame-core", core),
            html::footer::id("frame-footer",
                html::div::class("footer-content",
                    footer_content(state)
                )
            ),
        )
    )
}

//============ Private Helpers ===============================================

fn frame_nav_content(state: &RequestState, nav: Nav) -> impl Content + '_ {
    (
        html::a::class("frame-nav-brand", route::Home::link(state), (
            html::img::attrs((
                attr::src(route::assets::BrandLogo::link(state)),
                attr::width(64),
                attr::alt(i18n::term::nav::home(state)),
            )),
            html::button(
                "button",
                (
                    attr::class("navbar-toggler"),
                    attr::data("toggle", "collapse"),
                    attr::data("target", "#frame-nav-content"),
                    attr::aria("controls", "frame-nav-content"),
                    attr::aria("expanded", "false"),
                    attr::aria("label", i18n::term::nav::toggle_nav_bar(state)),
                ),
                html::span::class("navbar-toggler-icon", ())
            ),
            html::div::id_class(
                "frame-nav-content",
                ["frame-nav-content", "collapse"],
                (
                    html::ul::class("frame-nav-chapters", (
                        html::li::attrs(
                            matches!(nav, Nav::Home).then_some(
                                attr::class("active")
                            ),
                            html::a(route::Home::link(state),
                                i18n::term::nav::home(state)
                            )
                        ),
                        html::li::attrs(
                            matches!(nav, Nav::Browse).then_some(
                                attr::class("active")
                            ),
                            html::a("#",
                                i18n::term::nav::browse(state)
                            )
                        ),
                        html::li::attrs(
                            matches!(nav, Nav::Map).then_some(
                                attr::class("active")
                            ),
                            html::a("#",
                                i18n::term::nav::map(state)
                            )
                        ),
                        html::li::attrs(
                            matches!(nav, Nav::Documentation).then_some(
                                attr::class("active")
                            ),
                            html::a("#",
                                i18n::term::nav::documentation(state)
                            )
                        ),
                    )),
                    frame_nav_search(state),
                    frame_nav_lang(state),
                )
            )
        )),
    )
}

fn frame_nav_search(_state: &RequestState) -> impl Content + '_ {
    html::div::class("frame-nav-search", (
    ))
}

fn frame_nav_lang(_state: &RequestState) -> impl Content + '_ {
    ()
}

fn footer_content(_state: &RequestState) -> impl Content {
    ()
}


//------------ Frame ---------------------------------------------------------

struct Frame<Cont>(Cont);

impl<Cont: Content> Into<Body> for Frame<Cont> {
    fn into(self) -> Body {
        self.0.render().into()
    }
}

impl<Cont: Content> Page for Frame<Cont> { }

