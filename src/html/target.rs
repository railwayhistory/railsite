//! Basic assembling of HTML.

use std::fmt;
use hyper::StatusCode;
use hyper::http::response::Builder as ResponseBuilder;
use crate::http::Response;
use crate::i18n::Lang;


//------------ Target --------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Target {
    content: String,
    lang: Lang,
}

impl Target {
    pub fn new(lang: Lang) -> Self {
        Target {
            content: String::new(),
            lang
        }
    }

    pub fn html<F: FnOnce(&mut Content)>(mut self, op: F) -> Self {
        let lang = self.lang;
        self.content.push_str("<!DOCTYPE html>");
        Element::start("html", &mut self).attr("lang", lang.code())
            .content(op);
        self
    }

    pub fn into_string(self) -> String {
        self.content
    }

    pub fn into_response(self, status: StatusCode) -> Response {
        ResponseBuilder::new()
        .status(status)
        .header("Content-Type", "text/html;charset=utf-8")
        .header("Set-Cookie", self.lang.cookie())
        .body(self.into_string().into())
        .unwrap()
    }

    pub fn into_ok(self) -> Response {
        self.into_response(StatusCode::OK)
    }
}

impl Default for Target {
    fn default() -> Self {
        Target::new(Lang::default())
    }
}


//------------ Element -------------------------------------------------------

pub struct Element<'a> {
    tag: &'a str,
    target: &'a mut Target,
    empty: bool,
}

impl<'a> Element<'a> {
    fn start(tag: &'a str, target: &'a mut Target) -> Self {
        target.content.push('<');
        target.content.push_str(tag);
        Element { tag, target, empty: true }
    }

    pub fn attr(
        self, name: &str, value: impl RenderText
    ) -> Self {
        self.target.content.push(' ');
        self.target.content.push_str(name);
        self.target.content.push_str("=\"");
        value.render(&mut Text::attr(self.target));
        self.target.content.push('"');
        self
    }

    pub fn content<F: FnOnce(&mut Content)>(mut self, op: F) {
        self.empty = false;
        self.target.content.push('>');
        op(&mut Content::start(self.target))
    }

    pub fn get_content(&mut self) -> Content {
        if self.empty {
            self.empty = false;
            self.target.content.push('>');
        }
        Content::start(self.target)
    }

    pub fn render(self, what: impl RenderContent) {
        self.content(|cont| what.render(cont));
    }

    pub fn text<R: RenderText>(&mut self, text: R) {
        self.get_content().text(text)
    }

    pub fn touch(mut self) {
        self.target.content.push('>');
        self.empty = false;
    }
}

/// # Commonly used attributes
///
impl<'a> Element<'a> {
    pub fn alt(self, value: impl RenderText) -> Self {
        self.attr("alt", value)
    }

    pub fn class(self, value: &str) -> Self {
        self.attr("class", value)
    }

    pub fn href(self, value: impl RenderText) -> Self {
        self.attr("href", value)
    }

    pub fn id(self, value: impl RenderText) -> Self {
        self.attr("id", value)
    }

    pub fn src(self, value: impl RenderText) -> Self {
        self.attr("src", value)
    }
}

impl<'a> Drop for Element<'a> {
    fn drop(&mut self) {
        if self.empty {
            self.target.content.push_str("/>")
        }
        else {
            self.target.content.push_str("</");
            self.target.content.push_str(self.tag);
            self.target.content.push('>');
        }
    }
}


//------------ Content -------------------------------------------------------

pub struct Content<'a> {
    target: &'a mut Target,
}

impl<'a> Content<'a> {
    fn start(target: &'a mut Target) -> Self {
        Content { target }
    }

    pub fn render(&mut self, what: impl RenderContent) {
        what.render(self);
    }

    pub fn element<'s>(&'s mut self, tag: &'s str) -> Element<'s> {
        Element::start(tag, self.target)
    }

    pub fn text<R: RenderText>(&mut self, text: R) {
        text.render(&mut Text::pcdata(self.target))
    }

    pub fn raw<R: RenderText>(&mut self, text: R) {
        text.render(&mut Text::raw(self.target))
    }

    pub fn push_raw(&mut self, op: impl FnOnce(&mut String)) {
        op(&mut self.target.content)
    }

    pub fn write_fmt(&mut self, args: fmt::Arguments) {
        self.text(args)
    }

    pub fn lang(&self) -> Lang {
        self.target.lang
    }

    pub fn ends_with(&self, suffix: &str) -> bool {
        self.target.content.ends_with(suffix)
    }
}


/// # Commonly Encountered Elements
///
impl<'a> Content<'a> {
    pub fn a(&mut self) -> Element { self.element("a") }
    pub fn br(&mut self) -> Element { self.element("br") }
    pub fn dd(&mut self) -> Element { self.element("dd") }
    pub fn div(&mut self) -> Element { self.element("div") }
    pub fn dl(&mut self) -> Element { self.element("dl") }
    pub fn dt(&mut self) -> Element { self.element("dt") }
    pub fn h1(&mut self) -> Element { self.element("h1") }
    pub fn h2(&mut self) -> Element { self.element("h2") }
    pub fn h3(&mut self) -> Element { self.element("h3") }
    pub fn h4(&mut self) -> Element { self.element("h4") }
    pub fn i(&mut self) -> Element { self.element("i") }
    pub fn img(&mut self) -> Element { self.element("img") }
    pub fn li(&mut self) -> Element { self.element("li") }
    pub fn p(&mut self) -> Element { self.element("p") }
    pub fn section(&mut self) -> Element { self.element("section") }
    pub fn span(&mut self) -> Element { self.element("span") }
    pub fn strong(&mut self) -> Element { self.element("strong") }
    pub fn table(&mut self) -> Element { self.element("table") }
    pub fn tbody(&mut self) -> Element { self.element("tbody") }
    pub fn td(&mut self) -> Element { self.element("td") }
    pub fn th(&mut self) -> Element { self.element("th") }
    pub fn thead(&mut self) -> Element { self.element("thead") }
    pub fn tr(&mut self) -> Element { self.element("tr") }
    pub fn tt(&mut self) -> Element { self.element("tt") }
    pub fn ul(&mut self) -> Element { self.element("ul") }

    pub fn linked_script(&mut self, src: impl RenderText) {
        self.element("script").attr("src", src).touch();
    }
}


//------------ RenderContent -------------------------------------------------

pub trait RenderContent {
    fn render(self, content: &mut Content);
}


//------------ Text ----------------------------------------------------------

pub struct Text<'a> {
    target: &'a mut Target,
    escape: TextEscape
}

impl<'a> Text<'a> {
    fn new(target: &'a mut Target, escape: TextEscape) -> Self {
        Text { target, escape }
    }

    fn attr(target: &'a mut Target) -> Self {
        Text::new(target, TextEscape::Attr)
    }

    fn pcdata(target: &'a mut Target) -> Self {
        Text::new(target, TextEscape::Pcdata)
    }

    fn raw(target: &'a mut Target) -> Self {
        Text::new(target, TextEscape::Raw)
    }

    pub fn push(&mut self, ch: char) {
        match self.escape.replace_char(ch) {
            Some(s) => self.target.content.push_str(s),
            None => self.target.content.push(ch)
        }
    }

    pub fn push_str(&mut self, s: &str) {
        self.escape.push_escaped(s, &mut self.target.content);
    }

    pub fn write_fmt(&mut self, args: fmt::Arguments) {
        fmt::Write::write_fmt(self, args).unwrap();
    }

    pub fn lang(&self) -> Lang {
        self.target.lang
    }
}

impl<'a> fmt::Write for Text<'a> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.push_str(s);
        Ok(())
    }
}


//------------ RenderText ----------------------------------------------------

pub trait RenderText: Sized {
    fn render(self, target: &mut Text);

    fn format(self) -> String {
        let mut target = Target::default();
        self.render(&mut Text::raw(&mut target));
        target.into_string()
    }
}

impl<F: FnOnce(&mut Text)> RenderText for F {
    fn render(self, target: &mut Text) {
        (self)(target)
    }
}

impl<'a> RenderText for &'a str {
    fn render(self, target: &mut Text) {
        target.push_str(self)
    }
}

impl<'a> RenderText for fmt::Arguments<'a> {
    fn render(self, target: &mut Text) {
        write!(target, "{}", self)
    }
}


//----------- Functions ------------------------------------------------------

pub fn empty(_: &mut Content) {
}


//----------- Displayed ------------------------------------------------------

pub fn display<T>(t: T) -> Displayed<T> { Displayed(t) }

pub struct Displayed<T>(T);

impl<T: fmt::Display> RenderText for Displayed<T> {
    fn render(self, target: &mut Text) {
        write!(target, "{}", self.0)
    }
}


//------------ TextEscape ----------------------------------------------------

#[derive(Clone, Copy, Debug)]
enum TextEscape {
    Attr,
    Pcdata,
    Raw
}

impl TextEscape {
    fn replace_char(self, ch: char) -> Option<&'static str> {
        match self {
            TextEscape::Attr => {
                match ch {
                    '<' => Some("&lt;"),
                    '>' => Some("&gt;"),
                    '"' => Some("&quot;"),
                    '\'' => Some("&apos;"),
                    '&' => Some("&amp;"),
                    _ => None
                }
            }
            TextEscape::Pcdata => {
                match ch {
                    '<' => Some("&lt;"),
                    '&' => Some("&amp;"),
                    _ => None
                }
            }
            TextEscape::Raw => None
        }
    }

    fn push_escaped(self, mut s: &str, target: &mut String) {
        while !s.is_empty() {
            let mut iter = s.char_indices().map(|(idx, ch)| {
                (idx, self.replace_char(ch))
            });
            let end = loop {
                match iter.next() {
                    Some((idx, Some(repl))) => {
                        // Write up to index, write replacement string,
                        // break with index.
                        target.push_str(&s[0..idx]);
                        target.push_str(repl);
                        break idx;
                    }
                    Some((_, None)) => { }
                    None => {
                        return target.push_str(s);
                    }
                }
            };
            s = &s[end + 1..];
        }
    }
}

