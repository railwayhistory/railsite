use actix_web::Responder;
use actix_web::http::StatusCode;
use super::core::other;

pub fn not_found<'a>(path: &'a str) -> impl Responder {
    other("en",
        "404 Not Found",
        elements!(
            h1 {
                "404 Not Found"
            }
            p {
                tt { path }
                " not found on this server."
            }
        )
    ).status(StatusCode::NOT_FOUND)
}

