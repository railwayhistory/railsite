use htmlfn::html;
use htmlfn::core::Content;
use htmlfn::utils::iter;
use raildata::document::{entity, line};
use crate::page::{frame, snip};
use crate::route::Href;
use crate::state::RequestState;
use super::components::Chapter;

const CHAPTER: Chapter = Chapter::Lines;


pub fn page<'a>(
    entity: entity::Document<'a>, state: &'a RequestState
) -> impl frame::Page + 'a {
    frame::standard(state, CHAPTER.title(entity, state), (), (), (
        CHAPTER.headline(entity, state),
        html::table(
            iter(entity.xrefs().line_regions.iter().map(|(link, section)| {
                item(link.document(state.store()), section, state)
            }))
        )
    ))
}

fn item<'a>(
    line: line::Document<'a>,
    _section: &'a line::Section,
    state: &'a RequestState
) -> impl Content + 'a {
    html::tr((
        html::td(
            html::a(
                line.href(state),
                line.data().code().as_str(),
            )
        ),
        html::td(
            snip::line::line_title(line, state)
        )
    ))
}

