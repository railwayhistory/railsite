use htmlfn::core::{Content/*, iter, display*/};
use htmlfn::{elements, html_attrs};
use crate::i18n::Lang;
use crate::site::line::Line;
use super::core::other;

pub fn index<'a>(
    line: &'a Line<'a>,
    lang: Lang,
) -> impl Content + 'a {
    other(line.site(), lang,
        ("Line ", line.code()),
        elements!(
            @h1 {
                line.code();
                ". ";
                line.title();
            }
            @div(class="card") {
                @div(class="card-header") {
                    @ul(class="nav nav-tabs card-header-tabs", role="tablist") {
                        @li(class="nav-item") {
                            @a(
                                class="nav-link active",
                                id="points-tab",
                                "data-toggle" => "tab",
                                href="#points",
                                role="tab",
                                "aria-controls" => "points",
                                "aria-selected" => "true"
                            ) {
                                "Points"
                            }
                        }
                        @li(class="nav-item") {
                            @a(
                                class="nav-link",
                                id="history-tab",
                                "data-toggle" => "tab",
                                href="#history",
                                role="tab",
                                "aria-controls" => "history",
                            ) {
                                "History"
                            }
                        }
                    }
                }
                @div(class="card-body tab-content") {
                    @div(
                        class="tab-pane fade show active",
                        id="points",
                        role="tabpanel",
                        "aria-labelledby" => "points-tab"
                    ) {
                        points(line, lang)
                    }
                    @div(
                        class="tab-pane fade",
                        id="history",
                        role="tabpanel",
                        "aria-labelledby" => "history-tab"
                    ) {
                        ""//history(req, line)
                    }
                }
            }
        )
    )
}


//------------ points --------------------------------------------------------

fn points<'a>(
    _line: &'a Line, _lang: Lang
) -> impl Content + 'a {
    elements!(
        @table(class="table table-striped table-sm") {
            /*
            iter(line.points().map(|point| {
                let tr_class = if point.status().is_open() {
                    ""
                }
                else {
                    "text-secondary"
                };
                elements!(
                    @tr(class=tr_class) {
                        @td(class="text-center") {
                            @a(href="", class=tr_class) {
                                call(move || point.clone().name())
                            }
                        }
                    }
                )
            }))
            
            iter(line.points.iter().map(move |point_link| {
                let point = point_link.follow(req.state());
                let tr_class = if point.status().is_open() {
                    ""
                }
                else {
                    "text-secondary"
                };
                elements!(
                    @tr(class=tr_class) {
                        @td(class="text-center") {
                            km(req, line, point)
                        }
                        @td {
                            category(point)
                        }
                        @td {
                            @a(href=point::url(req, point),
                               class=tr_class
                            ) {
                                point.name()
                            }
                        }
                        @td {
                            connections(req, line, point, tr_class)
                        }
                    }
                )
            }))
            */
        }
    )
}

/*
fn km<'a>(
    req: &'a HttpRequest, line: &'a Line, point: &'a Point,
) -> &'a str {
    for event in point.events.iter().rev() {
        if let Some(ref location) = event.location.as_ref() {
            for (link, km) in location.0.iter() {
                if link.follow(req.state()).key() == line.key() {
                    return match km.as_ref() {
                        Some(value) => value.as_value().as_ref(),
                        None => "—"
                    }
                }
            }
        }
    }
    "?"
}

fn category<'a>(
    point: &'a Point
) -> impl Content + 'a {
    for event in point.events.iter().rev() {
        if let Some(ref category) = event.category {
            return Some(iter(
                category.iter().map(|item| {
                    let item = item.as_str();
                    let item = if let Some(idx) = item.find('.') {
                        &item[idx + 1..]
                    }
                    else { item };
                    (item, " ")
                })
            ))
        }
    }
    None
}

fn connections<'a>(
    req: &'a HttpRequest, line: &'a Line, point: &'a Point,
    tr_class: &'static str
) -> impl Content + 'a {
    let mut first = true;
    iter(point.lines.iter().map(move |conn| {
        let conn = conn.follow(req.state());
        if conn.key() != line.key() {
            Some(elements!(
                {
                    if first {
                        first = false;
                        None
                    }
                    else {
                        Some(", ")
                    }
                }
                @a(
                    href=line::url(req, conn),
                    class=tr_class,
                    title=line::title(req, conn)
                ) {
                    line::code(conn)
                }
            ))
        }
        else {
            None
        }
    }))
}


//------------ history -------------------------------------------------------

fn history<'a>(
    _req: &'a HttpRequest, line: &'a Line
) -> impl Content + 'a {
    elements!(
        @table(class="table table-borderless") {
            iter(line.events.iter().map(move |event| {
                elements!(
                    @tr {
                        @td {
                            iter(event.date.iter().map(|date| display(date)));
                        }
                        @td { "..." }
                    }
                )
            }))
        }
    )
}

*/
