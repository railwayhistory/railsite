//! Basic framework.
use std::fmt;
use xml::escape::{escape_str_attribute, escape_str_pcdata};


//------------ Text ----------------------------------------------------------

pub trait Text {
    fn render<W: fmt::Write>(&self, wr: &mut W) -> fmt::Result;

    fn render_string(&self) -> Result<String, fmt::Error> {
        let mut res = String::new();
        self.render(&mut res)?;
        Ok(res)
    }
}

impl<'a> Text for &'a str {
    fn render<W: fmt::Write>(&self, wr: &mut W) -> fmt::Result {
        wr.write_str(escape_str_pcdata(self).as_ref())
    }
}

impl Text for () {
    fn render<W: fmt::Write>(&self, _wr: &mut W) -> fmt::Result {
        Ok(())
    }
}

impl<A: Text, B: Text> Text for (A, B) {
    fn render<W: fmt::Write>(&self, wr: &mut W) -> fmt::Result {
        self.0.render(wr)?;
        self.1.render(wr)
    }
}

impl<A: Text, B: Text, C: Text> Text for (A, B, C) {
    fn render<W: fmt::Write>(&self, wr: &mut W) -> fmt::Result {
        self.0.render(wr)?;
        self.1.render(wr)?;
        self.2.render(wr)
    }
}

impl<A: Text, B: Text, C: Text, D: Text> Text for (A, B, C, D) {
    fn render<W: fmt::Write>(&self, wr: &mut W) -> fmt::Result {
        self.0.render(wr)?;
        self.1.render(wr)?;
        self.2.render(wr)?;
        self.3.render(wr)
    }
}

impl<A: Text, B: Text, C: Text, D: Text, E: Text> Text for (A, B, C, D, E) {
    fn render<W: fmt::Write>(&self, wr: &mut W) -> fmt::Result {
        self.0.render(wr)?;
        self.1.render(wr)?;
        self.2.render(wr)?;
        self.3.render(wr)?;
        self.4.render(wr)
    }
}


//------------ Name -----------------------------------------------------------

pub trait Name {
    fn render<W: fmt::Write>(&self, wr: &mut W) -> fmt::Result;
}

impl<'a> Name for &'a str {
    fn render<W: fmt::Write>(&self, wr: &mut W) -> fmt::Result {
        wr.write_str(self)
    }
}


//------------ AttrValue ------------------------------------------------------

pub trait AttrValue {
    fn render<W: fmt::Write>(&self, wr: &mut W) -> fmt::Result;
}

impl<'a> AttrValue for &'a str {
    fn render<W: fmt::Write>(&self, wr: &mut W) -> fmt::Result {
        wr.write_str(escape_str_attribute(self).as_ref())
    }
}


//------------ Element -------------------------------------------------------

pub trait Element {
    fn start<W: fmt::Write>(&self, wr: &mut W) -> fmt::Result;
    fn end<W: fmt::Write>(&self, wr: &mut W) -> fmt::Result;
}


//------------ Tag -----------------------------------------------------------

pub fn elem<N: Name>(tag: N) -> Tag<N> {
    Tag::new(tag)
}

pub struct Tag<N: Name>(N);

impl<N: Name> Tag<N> {
    pub fn new(tag: N) -> Self {
        Tag(tag)
    }

    pub fn attr<NN, V>(self, name: NN, value: V) -> Attr<Self, NN, V>
    where NN: Name, V: AttrValue {
        Attr::new(self, name, value)
    }
}

impl<N: Name> Element for Tag<N> {
    fn start<W: fmt::Write>(&self, wr: &mut W) -> fmt::Result {
        wr.write_str("<")?;
        self.0.render(wr)
    }

    fn end<W: fmt::Write>(&self, wr: &mut W) -> fmt::Result {
        wr.write_str("</")?;
        self.0.render(wr)?;
        wr.write_str(">")
    }
}

impl<N: Name> Text for Tag<N> {
    fn render<W: fmt::Write>(&self, wr: &mut W) -> fmt::Result {
        wr.write_str("<")?;
        self.0.render(wr)?;
        wr.write_str("/>")
    }
}


//------------ Attr ----------------------------------------------------------

pub struct Attr<E: Element, N: Name, V: AttrValue> {
    element: E,
    name: N,
    value: V
}

impl<E: Element, N: Name, V: AttrValue> Attr<E, N, V> {
    pub fn new(element: E, name: N, value: V) -> Self {
        Attr { element, name, value }
    }
}

impl<E: Element, N: Name, V: AttrValue> Element for Attr<E, N, V> {
    fn start<W: fmt::Write>(&self, wr: &mut W) -> fmt::Result {
        self.element.start(wr)?;
        wr.write_str(" ")?;
        self.name.render(wr)?;
        wr.write_str("=\"")?;
        self.value.render(wr)?;
        wr.write_str("\"")
    }

    fn end<W: fmt::Write>(&self, wr: &mut W) -> fmt::Result {
        self.element.end(wr)
    }
}

impl<E: Element, N: Name, V: AttrValue> Text for Attr<E, N, V> {
    fn render<W: fmt::Write>(&self, wr: &mut W) -> fmt::Result {
        self.start(wr)?;
        wr.write_str("/>")
    }
}


//------------ Content -------------------------------------------------------

pub struct Content<E: Element, C: Text>(E, C);

impl<E: Element, C: Text> Content<E, C> {
    pub fn new(element: E, content: C) -> Self {
        Content(element, content)
    }
}

impl<E: Element, C: Text> Text for Content<E, C> {
    fn render<W: fmt::Write>(&self, wr: &mut W) -> fmt::Result {
        self.0.start(wr)?;
        wr.write_str(">")?;
        self.1.render(wr)?;
        self.0.end(wr)
    }
}
