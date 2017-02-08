//! Tools for processing HTTP requests.

use std::{io, ops};
use futures::future::{Future, FutureResult, IntoFuture};
use hyper::Error;
use hyper::server::{Request, Response, Service};
use hyper::header::{ContentLength, ContentType};
use hyper::status::StatusCode;


pub struct Context<'a, E: 'a> {
    request: &'a Request,
    env: &'a E,
}

impl<'a, E> Context<'a, E> {
    pub fn new(request: &'a Request, env: &'a E) -> Self {
        Context {
            request: request,
            env: env
        }
    }

    pub fn request(&self) -> &'a Request {
        self.request
    }

    pub fn env(&self) -> &'a E {
        self.env
    }
}

impl<'a, E: 'a> ops::Deref for Context<'a, E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        self.env
    }
}


//----------- HtmlResponse ---------------------------------------------------

pub struct HtmlResponse {
    response: Response,
    body: Vec<u8>,
}

impl HtmlResponse {
    pub fn new(status: StatusCode) -> Self {
        HtmlResponse {
            response: Response::new().with_status(status)
                                     .with_header(ContentType::html()),
            body: Vec::new(),
        }
    }

    pub fn new_ok() -> Self {
        HtmlResponse::new(StatusCode::Ok)
    }

    pub fn new_not_found() -> Self {
        HtmlResponse::new(StatusCode::NotFound)
    }

    pub fn new_forbidden() -> Self {
        HtmlResponse::new(StatusCode::Forbidden)
    }

    pub fn finalize(self) -> Response {
        self.response.with_header(ContentLength(self.body.len() as u64))
                     .with_body(self.body)
    }
}

impl From<HtmlResponse> for Response {
    fn from(res: HtmlResponse) -> Self {
        res.finalize()
    }
}

impl io::Write for HtmlResponse {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.body.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.body.flush()
    }
}


//------------ View ----------------------------------------------------------

pub trait View<E> {
    type Future: Future<Item=Response, Error=Error>;

    fn call(&self, context: Context<E>) -> Self::Future;
}

impl<Env, F, R, Err>  View<Env> for F 
        where F: Fn(Context<Env>) -> Result<R, Err>,
              R: Into<Response>, Err: Into<Error> {
    type Future = FutureResult<Response, Error>;

    fn call(&self, context: Context<Env>) -> Self::Future {
        self(context).map(Into::into).map_err(Into::into).into_future()
    }
}


//------------ Site ----------------------------------------------------------

pub struct Site<E, V> {
    env: E,
    root: V,
}

impl<E, V> Site<E, V> {
    pub fn new(env: E, root: V) -> Self {
        Site {
            env: env,
            root: root
        }
    }
}

impl<E, V: View<E>> Service for Site<E, V> {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = V::Future;

    fn call(&self, request: Self::Request) -> Self::Future {
        let context = Context::new(&request, &self.env);
        self.root.call(context)
    }
}

