use actix_web::Responder;
use htmlfn::core::display;
use super::core::other;

pub fn index(doc_num: usize) -> impl Responder {
    other("en",
        "The Railway History Database",
        elements!(
            h1 {
                "The Railway History Database"
            }
            p {
                "Currently containing ",
                display(doc_num),
                " documents"
            }
        )
    )
}

