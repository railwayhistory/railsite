use horrorshow::html;
use super::core::other;
use crate::http::Request;
use crate::i18n::Lang;
/*
use raildata::document::Line;
use crate::app::HttpRequest;
use crate::document::{line, point};
*/


//------------ index ---------------------------------------------------------

pub fn index(request: &Request, doc_num: usize) -> String {
    other(request,
        "The Railway History Database",
        html! {
            h1 {
                : "The Railway History Database";
            }
            p {
                @ if request.lang() == Lang::De {
                    : "Enthält zurzeit ";
                    : doc_num;
                    : " Dokumente";
                } else {
                    : "Currently containing ";
                    : doc_num;
                    : " documents";
                }
            }
        }
    )
}


/*
//------------ lines ---------------------------------------------------------

pub fn lines<'a>(
    req: &'a HttpRequest,
    lines: impl Iterator<Item=&'a Line> + 'a,
) -> impl Content + 'a {
    other("en",
        "Line Index",
        elements!(
            @h1 { "Line Index" }
            @table(class="table") {
                @tbody() { 
                    iter(lines.map(move |item| lines_row(req, item)))
                }
            }
        )
    )
}

fn lines_row<'a>(
    req: &'a HttpRequest,
    line: &'a Line,
) -> impl Content + 'a {
    elements!(
        @tr {
            @td {
                @a(href=line::url(req, line)) {
                    line::code(line)
                }
            }
            @td {
                iter({
                    let mut first = true;
                    line.junctions(req.state()).map(move |pt| {
                        elements!(
                            {
                                if first {
                                    first = false;
                                    ""
                                }
                                else { "\u{a0}– " }
                            }
                            @a(class="text-dark", href=point::url(req, pt)) {
                                pt.name()
                            }
                        )
                    })
                })
            }
        }   
    )
}
*/
