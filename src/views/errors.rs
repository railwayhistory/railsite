use actix_web::HttpResponse;
use htmlfn::Content;
use crate::app::HttpRequest;
use crate::html;

pub fn not_found(request: &HttpRequest) -> HttpResponse {
    HttpResponse::NotFound().body(
        html::errors::not_found(request.request().uri().path()).into_string()
    )
}
