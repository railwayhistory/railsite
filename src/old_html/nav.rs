//! The navigation bar.
//!
use horrorshow::{html, TemplateBuffer};
use crate::i18n::Lang;
use crate::site::Site;


//------------ Nav -----------------------------------------------------------

/// Something that can render the navigation bar.
pub trait Nav {
    /// Renders the navigation bar for the given request.
    fn render_once(site: &Site,tmpl: &mut TemplateBuffer);
}


//------------ Other ---------------------------------------------------------

/// The navigation bar when no category is selected.
pub struct Other;

impl Nav for Other {
    fn render_once(site: &Site, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            ul(class="frame-nav-parts") {
                li {
                    a(href=site.link_home()) {
                        span(class="icon", title="Home") {
                            i(class="frame-icon-home") { }
                        }
                        span(class="text") {
                            @ if site.lang() == Lang::De {
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

