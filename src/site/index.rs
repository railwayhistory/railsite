use crate::html;
use crate::html::target::RenderText;
use crate::http::{GetRequest, Response};
use super::Site;

//------------ Site ----------------------------------------------------------

impl Site {
    pub(super) fn process_index(
        &self, request: GetRequest
    ) -> Result<Response, GetRequest> {
        if request.path().segment().is_empty() {
            Ok(html::index::home(self).into_ok())
        }
        else {
            Err(request)
        }
    }

    pub fn link_home<'s>(&'s self) -> impl RenderText + 's {
        self.link("")
    }
}

