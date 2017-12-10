use hyper::server::Request;
use htmlfn::html;
use super::core::core;

pub fn not_found(request: &Request) -> String {
    core("en",
        "404 Not Found",
        "nav",
        (
            html::h1().content(
                "404 Not Found"
            ),
            html::p().content((
                html::tt().content(
                    request.path()
                ),
                " not found on this server."
            ))
        )
    )
}

