use std::ops::Deref;
use raildata::document;
use crate::html;
use crate::http::{ContentExt, Request, Response};
use crate::site::Site;


pub fn process(
    site: &Site, line: &document::Line, mut request: Request
) -> Response {
    let line = Line::new(site, line);
    match request.path_mut().next() {
        None => {
            request.get(|request| {
                Ok(html::line::index(&line, request.lang()).ok(request))
            }).unwrap_or_else(|request| site.method_not_allowed(request))
        }
        _ => site.not_found(request)
    }
}


pub struct Line<'a> {
    site: &'a Site,
    line: &'a document::Line
}

impl<'a> Line<'a> {
    fn new(site: &'a Site, line: &'a document::Line) -> Self {
        Line { site, line }
    }

    pub fn site(&self) -> &Site {
        self.site
    }

    pub fn line(&self) -> &document::Line {
        self.line
    }

    pub fn code<'s>(&'s self) -> impl Text + 's {
        let code = self.key().as_str();
        if code.starts_with("line.") && code.get(7..8) == Some(".") {
            let (country, code) = (&code[5..7], &code[8..]);
            (country.to_ascii_uppercase(), ("\u{a0}", code))
        }
        else {
            (String::new(), ("", code))
        }
    }

    pub fn title<'s>(&'s self) -> impl Text + 's {
        (self.first_junction(self.site.library()).name(),
            ("\u{a0}– ", self.last_junction(self.site.library()).name())
        )
    }

    pub fn points<'s>(&'s self) -> impl Iterator<Item=LinePoint<'s>> + 's {
        self.line.points.points.iter().map(move |link| {
            LinePoint {
                site: self.site,
                line: self.line,
                point: link.follow(self.site.library())
            }
        })
    }
}

impl<'a> Deref for Line<'a> {
    type Target = document::Line;

    fn deref(&self) -> &Self::Target {
        self.line()
    }
}


pub struct LinePoint<'a> {
    site: &'a Site,
    line: &'a document::Line,
    point: &'a document::Point,
}

impl<'a> Deref for LinePoint<'a> {
    type Target = document::Point;

    fn deref(&self) -> &Self::Target {
        self.point
    }
}

