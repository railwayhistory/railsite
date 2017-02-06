use futures::{Finished, finished};
use hyper::Error;
use hyper::header::{ContentLength, ContentType};
use hyper::server::{Service, Request, Response};

static PHRASE: &'static [u8] = b"Hello World!";

#[derive(Clone, Copy)]
pub struct Site;

impl Service for Site {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = Finished<Response, Error>;

    fn call(&self, _req: Request) -> Self::Future {
        finished(
            Response::new()
                .with_header(ContentLength(PHRASE.len() as u64))
                .with_header(ContentType::plaintext())
                .with_body(PHRASE)
        )
    }
}

