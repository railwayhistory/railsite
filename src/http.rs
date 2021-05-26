//! Tools for processing HTTP requests.

use std::ops;
use std::borrow::Cow;
use std::collections::HashMap;
use headers::{Cookie, HeaderMapExt};
use hyper::{Body, Method, StatusCode};
use hyper::http::uri::PathAndQuery;
use hyper::http::response::Builder as ResponseBuilder;
use json::JsonValue;
use url::form_urlencoded;
use url::percent_encoding::percent_decode;
use crate::i18n::Lang;


//------------ Request -------------------------------------------------------

pub struct Request {
    request: hyper::Request<Body>,
    path: RequestPath,
    query: HashMap<String, String>,
    lang: Lang,
}

impl Request {
    pub fn new(
        request: hyper::Request<Body>,
    ) -> Self {
        let query = form_urlencoded::parse(
            request.uri().query().unwrap_or("").as_bytes()
        ).into_owned().collect();
        let lang = Self::determine_lang(&request, &query);
        Request {
            path: RequestPath::from_request(&request),
            query, lang,
            request
        }
    }

    /// Determine the language.
    ///
    /// Returns the language and prepares the builder.
    fn determine_lang(
        request: &hyper::Request<Body>,
        query: &HashMap<String, String>,
    ) -> Lang {
        // If we have a "lang" attribute in the query, we use that -- this is
        // how we switch languages.
        if let Some(lang) = query.get("lang") {
            return Lang::from_code(lang)
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

    pub fn path(&self) -> &RequestPath {
        &self.path
    }

    pub fn path_mut(&mut self) -> &mut RequestPath {
        &mut self.path
    }

    pub fn query(&self) -> &HashMap<String, String> {
        &self.query
    }

    pub fn query_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.query
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

    pub fn get(self) -> Result<GetRequest, Self> {
        if self.is_get() {
            Ok(GetRequest(self))
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

    pub fn respond_json(
        &self,
        status: StatusCode,
        json: JsonValue
    ) -> Response {
        let mut body = Vec::new();
        json.write(&mut body).unwrap();
        self.respond_raw(status, "application/json", body)
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

    pub fn redirect(&self, target: String) -> Response {
        ResponseBuilder::new()
            .status(StatusCode::MOVED_PERMANENTLY)
            .header("Location", target)
            .body("".into())
            .unwrap()
    }

    pub fn ok(&self, body: Body) -> Response {
        self.respond(StatusCode::OK, body)
    }

    pub fn ok_json(&self, json: JsonValue) -> Response {
        self.respond_json(StatusCode::OK, json)
    }
}


//------------ GetRequest ----------------------------------------------------

pub struct GetRequest(Request);

impl From<GetRequest> for Request {
    fn from(src: GetRequest) -> Request {
        src.0
    }
}

impl ops::Deref for GetRequest {
    type Target = Request;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for GetRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


//------------ RequestPath ---------------------------------------------------

pub struct RequestPath {
    path: Result<PathAndQuery, String>,
    segment: (usize, usize),
}

impl RequestPath {
    fn from_request<B>(request: &hyper::Request<B>) -> Self {
        let path = if let Cow::Owned(some) = percent_decode(
            request.uri().path().as_bytes()
        ).decode_utf8_lossy() {
            Err(some)
        }
        else {
            Ok(request.uri().path_and_query().unwrap().clone())
        };
        let mut res = RequestPath {
            path,
            segment: (0, 0),
        };
        res.next_segment();
        res
    }

    pub fn full(&self) -> &str {
        match self.path.as_ref() {
            Ok(path) => path.path(),
            Err(path) => path.as_str()
        }
    }

    pub fn remaining(&self) -> &str {
        &self.full()[self.segment.0..]
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

    /// Returns the next segment if it is the final segment.
    ///
    /// If there are more segments after the next segment, returns the entire
    /// remaining path as an error.
    pub fn next_and_last(&mut self) -> Result<Option<&str>, &str> {
        if !self.next_segment() {
            return Ok(None)
        }
        let path = self.full();
        if self.segment.1 == path.len()
            || (self.segment.1 + 1 == path.len()
                    && path.as_bytes()[self.segment.1] == b'/'
                )
        {
            return Ok(Some(self.segment()))
        }
        else {
            Err(self.remaining())
        }
    }
}


//------------ Response ------------------------------------------------------

pub type Response = hyper::Response<Body>;

