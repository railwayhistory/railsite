use horrorshow::html;
use crate::http::Request;
use crate::i18n::Lang;
use super::core::other;

pub fn not_found(
    request: &Request
) -> String {
    other(request,
        "404 Not Found",
        html! {
            h1 {
                @ if request.lang() == Lang::De {
                    : "404 Nicht gefunden"
                }
                else {
                    : "404 Not Found"
                }
            }
            p {
                @ if request.lang() == Lang::De {
                    : "Der Pfad ";
                    tt { : request.path().full() }
                    : " konnte auf dem Server nicht gefunden werden."
                }
                else {
                    : "Path ";
                    tt { : request.path().full() }
                    : " not found on this server."
                }
            }
        }
    )
}

pub fn method_not_allowed(
    request: &Request,
) -> String {
    other(request,
        "405 Method Not Allowed",
        html! {
            h1 {
                @ if request.lang() == Lang::De {
                    : "405 Methode nicht zugelassen"
                }
                else {
                    : "405 Method Not Allowed"
                }
            }
            p {
                @ if request.lang() == Lang::De {
                    : "Die Methode ";
                    tt { : request.method().as_str() }
                    : " ist für den Pfad ";
                    tt { : request.path().full() }
                    : " nicht zugelassen."
                }
                else {
                    : "The method ";
                    tt { : request.method().as_str() }
                    : " is not allowed at ";
                    tt { : request.path().full() }
                    : "."
                }
            }
        }
    )
}

