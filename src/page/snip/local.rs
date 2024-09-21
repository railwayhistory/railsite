use htmlfn::core::{Content, Target, Text};
use raildata::types::LocalCode;
use crate::state::RequestState;


//------------ opt_local_code ------------------------------------------------

pub fn opt_local_code<'a>(
    code: Option<impl AsRef<LocalCode>>, _state: &'a RequestState
) -> impl Text + Content + 'a {
    LocalCodeText(code.map(|x| *x.as_ref()))
}

struct LocalCodeText(Option<LocalCode>);
impl Content for LocalCodeText {
    fn render_content(self, target: &mut Target) {
        if let Some(code) = self.0 {
            target.append_slice(code.as_str().as_bytes());
        }
    }
}

impl Text for LocalCodeText { }


