use horrorshow::Template;
use crate::html;
use crate::http::{Request, Response};

pub fn process(request: Request) -> Result<Response, Request> {
    if request.path().segment().is_empty() {
        Ok(request.ok(
            html::index::index(&request, request.library().len()).into()
        ))
    }
    else {
        Err(request)
    }
}


pub fn home(_request: &Request) -> impl Template {
    "/" 
}
