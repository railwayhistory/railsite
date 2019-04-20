use htmlfn::core::Content;
use super::core::other;
use raildata::document::{Line};
use raildata::library::Library;

pub fn index<'a>(line: &'a Line, _library: &'a Library) -> impl Content + 'a {
    let code = match line.code() {
        Ok((country, code)) => (country, (" ", code)),
        Err(code) => ("", ("", code))
    };
    other("en",
        ("Line ", code),
        elements!(
            h1() {
                "Line ";
                code;
            }
            p() {
                { format!("{:?}", line) }
            }
        )
    )
}

