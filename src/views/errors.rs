use actix_web::Responder;
use crate::app::HttpRequest;
use crate::html;

pub fn not_found(request: &HttpRequest) -> impl Responder {
    html::errors::not_found(request.request().uri().path())
}
