use htmlfn::core::{Content, iter};
use raildata::document::Line;
use raildata::library::Library;
use super::core::other;

pub fn index(doc_num: usize) -> impl Content {
    other("en",
        "The Railway History Database",
        elements!(
            h1() {
                "The Railway History Database"
            }
            p() {
                "Currently containing ";
                { htmlfn::core::display(doc_num) }
                " documents";
            }
        )
    )
}


//------------ lines ---------------------------------------------------------

pub fn lines<'a>(
    lines: impl Iterator<Item=&'a Line> + 'a,
    library: &'a Library
) -> impl Content + 'a {
    other("en",
        "Line Index",
        elements!(
            h1() { "Line Index" }
            table() {
                tbody() { 
                    {
                        iter(lines.map(move |item| line(item, library)))
                    }
                }
            }
        )
    )
}

fn line<'a>(line: &'a Line, library: &'a Library) -> impl Content + 'a {
    elements!(
        tr() {
            td() {
                {
                    match line.code() {
                        Ok((country, code)) => {
                            (country.to_ascii_uppercase(), ("\u{a0}", code))
                        }
                        Err(code) => (String::new(), ("", code))
                    }
                }
            }
            td() { {
                iter({
                    let mut first = true;
                    line.junctions(library).map(move |pt| {
                        (
                            if first {
                                first = false;
                                ""
                            }
                            else { "\u{a0}– " },
                            pt.name()
                        )
                    })
                })
            } }
        }   
    )
}
