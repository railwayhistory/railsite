//! Tools for processing HTTP requests.

use std::{fmt, io};
use std::borrow::Cow;
use std::str::FromStr;
use futures::future::{Future, FutureResult, IntoFuture};
use hyper::Error;
use hyper::server as hyper;
use hyper::server::Service;
use hyper::header::{ContentLength, ContentType};
use hyper::status::StatusCode;
use url::percent_encoding::percent_decode;

pub use hyper::server::Response;


//------------ Request -------------------------------------------------------

pub struct Request<'a> {
    request: &'a hyper::Request,
    path_pos: Option<usize>,
}

impl<'a> Request<'a> {
    pub fn new(request: &'a hyper::Request) -> Self {
        Request {
            request: request,
            path_pos: Some(0),
        }
    }

    pub fn next_segment(mut self) -> (Option<PathSegment<'a>>, Self) {
        let mut start = match self.path_pos {
            Some(start) => start,
            None => return (None, self),
        };
        let path = self.request.path();
        // Start beyond the length of the path signals the end.
        if start >= path.len() {
            return (None, self)
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
        let res = PathSegment::new(&path[start..end]);
        self.path_pos = Some(end);
        (Some(res), self)
    }
}

impl<'a> Request<'a> {
    /// Returns the complete path.
    pub fn path(&self) -> &str {
        self.request.path()
    }

    /// Returns the remaining portion of the path.
    pub fn remaining_path(&self) -> &str {
        match self.path_pos {
            Some(pos) => &self.request.path()[pos..],
            None => &""
        }
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

    pub fn ok() -> Self {
        HtmlResponse::new(StatusCode::Ok)
    }

    pub fn not_found() -> Self {
        HtmlResponse::new(StatusCode::NotFound)
    }

    pub fn forbidden() -> Self {
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

pub trait View<C> {
    type Future: Future<Item=Response, Error=Error>;

    fn call(&self, request: Request, context: &C) -> Self::Future;
}

impl<C, F, R, E>  View<C> for F 
        where F: Fn(Request, &C) -> Result<R, E>,
              R: Into<Response>, E: Into<Error> {
    type Future = FutureResult<Response, Error>;

    fn call(&self, request: Request, context: &C) -> Self::Future {
        self(request, context).map(Into::into).map_err(Into::into)
                              .into_future()
    }
}


//------------ Site ----------------------------------------------------------

pub struct Site<C, V> {
    context: C,
    root: V,
}

impl<C, V> Site<C, V> {
    pub fn new(context: C, root: V) -> Self {
        Site {
            context: context,
            root: root
        }
    }
}

impl<C, V: View<C>> Service for Site<C, V> {
    type Request = hyper::Request;
    type Response = hyper::Response;
    type Error = Error;
    type Future = V::Future;

    fn call(&self, request: Self::Request) -> Self::Future {
        self.root.call(Request::new(&request), &self.context)
    }
}

/*
impl<E: Clone, V: View<E>> NewService for Site<E, V> {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Instance = Self;

    fn new_service(&self) -> Result<Self::Instance, io::Error> {
        Ok(Self::new(self.env.clone(), self.root))
    }
}
*/


//------------ PathSegment ---------------------------------------------------

pub struct PathSegment<'a>(Result<Cow<'a, str>, Vec<u8>>);


impl<'a> PathSegment<'a> {
    pub fn new(s: &'a str) -> Self {
        PathSegment(
            match percent_decode(s.as_bytes()).if_any() {
                Some(bytes) => {
                    String::from_utf8(bytes)
                           .map(Cow::Owned)
                           .map_err(|err| err.into_bytes())
                }
                None => Ok(Cow::Borrowed(s))
            }
        )
    }

    pub fn as_bytes(&self) -> &[u8] {
        match self.0 {
            Ok(ref s) => s.as_bytes(),
            Err(ref err) => err
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self.0 {
            Ok(ref s) => Some(s),
            Err(_) => None
        }
    }

    pub fn match_str(&self) -> &str {
        match self.0 {
            Ok(ref s) => s,
            Err(_) => ""
        }
    }

    pub fn try_as<T: FromStr>(&self) -> Option<T> {
        let s = match self.0 {
            Ok(ref s) => s,
            Err(_) => return None
        };
        FromStr::from_str(s).ok()
    }
}

impl<'a> fmt::Display for PathSegment<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Ok(ref s) => f.write_str(s),
            Err(ref err) => {
                f.write_str(&String::from_utf8_lossy(err))
            }
        }
    }
}

