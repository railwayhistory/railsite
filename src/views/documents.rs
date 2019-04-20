use actix_web::HttpResponse;
use htmlfn::core::Content;
use raildata::document::{Document, Line};
use raildata::types::Key;
use crate::{html, views};
use crate::app::HttpRequest;

pub fn document(req: &HttpRequest) -> HttpResponse {
    let link = Key::from_string(
        req.match_info().query("key").unwrap()
    ).ok().and_then(|key| req.state().get(&key));
    let link = match link {
        Some(link) => link,
        None => return views::errors::not_found(req)
    };
    match req.state().resolve(link) {
        Document::Line(inner) => line(req, inner),
        _ => unimplemented!()
    }
}

fn line(req: &HttpRequest, line: &Line) -> HttpResponse {
    HttpResponse::Ok().body(html::line::index(line, req.state()).into_string())
}

