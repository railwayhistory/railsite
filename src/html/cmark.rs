//! Converting CommonMark to HTML.

use pulldown_cmark::html;
use pulldown_cmark::{Event, Parser, Options, Tag};
use crate::site::Site;
use super::target::{Content, RenderText};

pub fn render(cont: &mut Content, cmark: &str, site: &Site) {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(cmark, options).map(|event| {
        match event {
            Event::Start(tag) => {
                match tag {
                    Tag::Link(link_type, destination, title) => {
                        let destination = if
                            let Some(document) = site.library().get(
                                destination.as_ref()
                            )
                        {
                            site.link_document(
                                document.follow(site.library())
                            ).format().into()
                        }
                        else {
                            destination
                        };
                        Event::Start(Tag::Link(link_type, destination, title))
                    }
                    _ => Event::Start(tag)
                }
            }
            _ => event
        }
    });
    cont.push_raw(|target| html::push_html(target, parser))
}

