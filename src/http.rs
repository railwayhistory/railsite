//! Tools for processing HTTP requests.

use std::io;
use std::borrow::Cow;
use std::sync::Arc;
use headers::{Cookie, HeaderMapExt};
//use horrorshow::{Template, html};
use hyper::{Body, Method, StatusCode};
use hyper::http::uri::PathAndQuery;
use hyper::http::response::Builder as ResponseBuilder;
use raildata::library::Library;
use url::form_urlencoded;
use url::percent_encoding::percent_decode;
use crate::i18n::Lang;


//------------ Request -------------------------------------------------------

pub struct Request {
    request: hyper::Request<Body>,
    path: RequestPath,
    lang: Lang,
    library: Library,
    //base: Arc<String>,
}

impl Request {
    pub fn new(
        request: hyper::Request<Body>,
        library: Library,
        _base: Arc<String>,
    ) -> Self {
        Request {
            path: RequestPath::from_request(&request),
            lang: Self::determine_lang(&request),
            request,
            library,
            //base
        }
    }

    pub fn library(&self) -> &Library {
        &self.library
    }

    /// Determine the language.
    ///
    /// Returns the language and prepares the builder.
    fn determine_lang(
        request: &hyper::Request<Body>
    ) -> Lang {
        // If we have a "lang" attribute in the query, we use that -- this is
        // how we switch languages.
        for (key, value) in form_urlencoded::parse(
            request.uri().query().unwrap_or("").as_bytes()
        ) {
            if key == "lang" {
                return Lang::from_code(value.as_ref())
            }
        }

        // If we have a "lang" cookie, we use that.
        if let Some(cookies) = request.headers().typed_get::<Cookie>() {
            if let Some(lang) = cookies.get("lang") {
                return Lang::from_code(lang)
            }
        }

        // Otherwise we will do the default for now.
        Lang::default()
    }

    /// Returns the requested language.
    pub fn lang(&self) -> Lang {
        self.lang
    }

    /// Returns the complete path.
    pub fn path(&self) -> &RequestPath {
        &self.path
    }

    pub fn path_mut(&mut self) -> &mut RequestPath {
        &mut self.path
    }

    /// Returns the method of this request.
    pub fn method(&self) -> &Method {
        self.request.method()
    }

    /// Returns whether the request is a GET request.
    pub fn is_get(&self) -> bool {
        self.request.method() == Method::GET
    }

    /// Returns whether the request is a GET request.
    pub fn is_post(&self) -> bool {
        self.request.method() == Method::POST
    }

    pub fn get<F>(self, op: F) -> Result<Response, Self>
    where F: FnOnce(Request) -> Result<Response, Self> {
        if self.is_get() {
            op(self)
        }
        else {
            Err(self)
        }
    }

    pub fn respond(
        &self,
        status: StatusCode,
        body: Body,
    ) -> Response {
        ResponseBuilder::new()
            .status(status)
            .header("Content-Type", "text/html;charset=utf-8")
            .header("Set-Cookie", self.lang.cookie())
            .body(body)
            .unwrap()
    }

    pub fn respond_raw(
        &self,
        status: StatusCode,
        content_type: &str,
        body: impl Into<Body>
    ) -> Response {
        ResponseBuilder::new()
            .status(status)
            .header("Content-Type", content_type)
            .body(body.into())
            .unwrap()
    }

    pub fn ok(&self, body: Body) -> Response {
        self.respond(StatusCode::OK, body)
    }

    /*
    pub fn link<'s>(&'s self, path: impl Template + 's) -> impl Template + 's {
        html! {
            : self.base.as_str();
            : "/";
            :path;
        }
    }
    */
}


//------------ RequestPath ---------------------------------------------------

pub struct RequestPath {
    path: PathAndQuery,
    segment: (usize, usize),
}

impl RequestPath {
    fn from_request<B>(request: &hyper::Request<B>) -> Self {
        let path = if let Cow::Owned(some) = percent_decode(
            request.uri().path().as_bytes()
        ).decode_utf8_lossy() {
            PathAndQuery::from_maybe_shared(some).unwrap()
        }
        else {
            request.uri().path_and_query().unwrap().clone()
        };
        let mut res = RequestPath {
            path,
            segment: (0, 0),
        };
        res.next_segment();
        res
    }

    pub fn full(&self) -> &str {
        self.path.path()
    }

    pub fn remaining(&self) -> &str {
        &self.full()[self.segment.1..]
    }

    pub fn segment(&self) -> &str {
        &self.full()[self.segment.0..self.segment.1]
    }

    fn next_segment(&mut self) -> bool {
        let mut start = self.segment.1;
        let path = self.full();
        // Start beyond the length of the path signals the end.
        if start >= path.len() {
            return false;
        }
        // Skip any leading slashes. There may be multiple which should be
        // folded into one (or at least that’s what we do).
        while path.split_at(start).1.starts_with('/') {
            start += 1
        }
        // Find the next slash. If we have one, that’s the end of
        // our segment, otherwise, we go all the way to the end of the path.
        let end = path[start..].find('/').map(|x| x + start)
                                         .unwrap_or(path.len());
        self.segment = (start, end);
        true 
    }

    pub fn next(&mut self) -> Option<&str> {
        if self.next_segment() {
            Some(self.segment())
        }
        else {
            None
        }
    }
}


//------------ Response ------------------------------------------------------

pub type Response = hyper::Response<Body>;


//----------- HtmlResponse ---------------------------------------------------

pub struct HtmlResponse {
    status: StatusCode,
    body: Vec<u8>,
}

impl HtmlResponse {
    pub fn new(status: StatusCode) -> Self {
        HtmlResponse {
            status,
            body: Vec::new()
        }
    }

    pub fn ok() -> Self {
        HtmlResponse::new(StatusCode::OK)
    }

    pub fn not_found() -> Self {
        HtmlResponse::new(StatusCode::NOT_FOUND)
    }

    pub fn forbidden() -> Self {
        HtmlResponse::new(StatusCode::FORBIDDEN)
    }

    pub fn finalize(self) -> Response {
        hyper::Response::builder()
            .status(self.status)
            .header("Content-Type", "text/html;charset=utf-8")
            .body(self.body.into()).unwrap()
    }
}

impl From<HtmlResponse> for Response {
    fn from(html: HtmlResponse) -> Self {
        html.finalize()
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


/*
//------------ ContentExt ----------------------------------------------------

pub trait ContentExt: Sized {
    fn into_body(self) -> Body;

    fn into_response(self, status: StatusCode, request: Request) -> Response {
        request.respond(status, self.into_body())
    }

    fn ok(self, request: Request) -> Response {
        self.into_response(StatusCode::OK, request)
    }

    fn method_not_allowed(self, request: Request) -> Response {
        self.into_response(StatusCode::METHOD_NOT_ALLOWED, request)
    }

    fn not_found(self, request: Request) -> Response {
        self.into_response(StatusCode::NOT_FOUND, request)
    }
}

impl<C: Content + Sized> ContentExt for C {
    fn into_body(self) -> Body {
        let mut body = Vec::new();
        self.write(&mut body).unwrap();
        body.into()
    }
}
*/
