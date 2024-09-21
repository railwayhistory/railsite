use htmlfn::html;
use htmlfn::core::{Content, Text};
use htmlfn::utils::either;
use raildata::document::line;
use crate::route::Href;
use crate::state::RequestState;

pub fn title<'a>(
    line: line::Document<'a>, state: &'a RequestState
) -> impl Text + 'a {(
    line.data().code().as_str(), ". ", line_title(line, state),
)}

pub fn line_title<'a>(
    line: line::Document<'a>, state: &'a RequestState
) -> impl Text + 'a {
    let lang = state.lang().into();
    let title = line.title(lang);
    either(title.is_some(),
        || title,
        || (
            line.first_junction_name(state.store(), lang),
            "\u{202f}–\u{2009}",
            line.last_junction_name(state.store(), lang),
        )
    )
}


pub fn link<'a>(
    line: line::Document<'a>,
    state: &'a RequestState
) -> impl Content + 'a {
    html::a(
        line.href(state),
        title(line, state),
    )
}


pub fn code_link<'a>(
    line: line::Document<'a>,
    state: &'a RequestState
) -> impl Content + 'a {
    let jurisdiction = line.data().jurisdiction();
    html::a(
        line.href(state),
        html::span::title(
            (
                line.data().points.first_junction(
                    state.store()
                ).data().name_in_jurisdiction(jurisdiction),
                " – ",
                line.data().points.last_junction(
                    state.store()
                ).data().name_in_jurisdiction(jurisdiction),
            ),
            line.data().code().as_str()
        )
    )
}

