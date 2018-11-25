use actix_web::Responder;
use crate::app::HttpRequest;
use crate::html;

pub fn index(req: &HttpRequest) -> impl Responder {
    html::index::index(req.state().len())
}
