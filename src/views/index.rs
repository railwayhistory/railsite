use actix_web::HttpResponse;
use htmlfn::Content;
use raildata::document::Document;
use raildata::types::Key;
use crate::app::HttpRequest;
use crate::html;

pub fn home(req: &HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .body(html::index::index(req.state().len()).into_string())
}

pub fn lines(req: &HttpRequest) -> HttpResponse {
    let library = req.state();
    let country = req.match_info().get("country");
    let start = Key::from_string(match country {
        Some(country) => format!("line.{}.", country),
        None => "line.".into(),
    }).unwrap(); 

    let iter = library.iter_from(&start).map(|doc| match *doc {
        Document::Line(ref inner) => Some(inner),
        _ => None
    }).take_while(move |line| {
        match line {
            Some(line) => line.key().starts_with(start.as_str()),
            None => false
        }
    }).map(Option::unwrap);

    HttpResponse::Ok()
        .body(html::index::lines(iter, library).into_string())
}

