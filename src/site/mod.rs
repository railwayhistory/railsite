
pub mod errors;
pub mod home;
//mod site;
pub mod statics;
//pub mod line;

use crate::http::{Request, Response};

pub fn process(request: Request) -> Response {
    statics::process(request)
    .or_else(|request| home::process(request))
    .unwrap_or_else(|request| errors::not_found(&request))
}

