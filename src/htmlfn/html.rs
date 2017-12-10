use std::fmt;
use super::core::{self, Content, Text, AttrValue, Element};

pub type Attr<E, V> = core::Attr<E, &'static str, V>;
pub type Tag = core::Tag<&'static str>;


//============ Macros for Element Generation =================================

macro_rules! html_element {
    (
        $tag:expr => ($fn_name:ident, $struct_name:ident) {
            $( $inner:tt )*
        }
    ) => {
        pub fn $fn_name() -> $struct_name<Tag> {
            $struct_name(Tag::new($tag))
        }

        pub struct $struct_name<E: Element>(E);

        impl<E: Element> $struct_name<E> {
            html_attr!( $struct_name $( $inner )* );
        }

        impl<E: Element> Element for $struct_name<E> {
            fn start<W: fmt::Write>(&self, wr: &mut W) -> fmt::Result {
                self.0.start(wr)
            }

            fn end<W: fmt::Write>(&self, wr: &mut W) -> fmt::Result {
                self.0.end(wr)
            }
        }

        impl<E: Element> Text for $struct_name<E> {
            fn render<W: fmt::Write>(&self, wr: &mut W) -> fmt::Result {
                self.start(wr)?;
                wr.write_str("/>")
            }
        }
    }
}

macro_rules! html_attr {
    // Terminator.
    (
        $struct_name:ident
    ) => {
    };

    // attr(fn_name) -> attr(fn_name, attr_name)
    (
        $struct_name:ident attr($name:ident); $( $tail:tt )*
    ) => {
        html_attr!($struct_name attr($name, stringify!($name)); $( $tail )*);
    };

    // attr(fn_name, attr_name)
    (
        $struct_name:ident attr($name:ident, $attr_name:expr); $( $tail:tt )*
    ) => {
        pub fn $name<V: AttrValue>(self, value: V)
                                   -> $struct_name<Attr<Self, V>> {
            $struct_name(Attr::new(self, $attr_name, value))
        }

        html_attr!($struct_name $( $tail )*);
    };

    // global_attrs()
    (
        $struct_name:ident global_attrs(); $( $tail:tt)*
    ) => {
        html_attr!($struct_name
            attr(class);
            attr(id);
            attr(role);
            $( $tail )*
        );
    };

    // content()
    (
        $struct_name:ident content(); $( $tail:tt )*
    ) => {
        pub fn content<C: Text>(self, content: C) -> Content<Self, C> {
            Content::new(self, content)
        }

        html_attr!($struct_name $( $tail )* );
    };
}



//============ Actual Elements ===============================================

//------------ html ----------------------------------------------------------

pub fn html() -> Html<HtmlTag> {
    Html(HtmlTag)
}

pub struct Html<E: Element>(E);

impl<E: Element> Html<E> {
    html_attr!{Html 
        attr(lang);
        global_attrs();
        content();
    }
}

impl<E: Element> Element for Html<E> {
    fn start<W: fmt::Write>(&self, wr: &mut W) -> fmt::Result {
        self.0.start(wr)
    }

    fn end<W: fmt::Write>(&self, wr: &mut W) -> fmt::Result {
        self.0.end(wr)
    }
}

impl<E: Element> Text for Html<E> {
    fn render<W: fmt::Write>(&self, wr: &mut W) -> fmt::Result {
        self.start(wr)?;
        wr.write_str("/>")
    }
}

//------------ head ----------------------------------------------------------

html_element!{
    "head" => (head, Head) {
        global_attrs();
        content();
    }
}


//------------ link ----------------------------------------------------------

html_element!{
    "link" => (link, Link) {
        attr(rel);
        attr(href);
        global_attrs();
    }
}


//------------ meta ----------------------------------------------------------

html_element!{
    "meta" => (meta, Meta) {
        attr(charset);
        attr(name);
        attr(value, "content");
        global_attrs();
        content();
    }
}


//------------ title ---------------------------------------------------------

html_element!{
    "title" => (title, Title) {
        global_attrs();
        content();
    }
}


//------------ body ----------------------------------------------------------

html_element!{
    "body" => (body, Body) {
        global_attrs();
        content();
    }
}


//------------ h1 ------------------------------------------------------------

html_element!{
    "h1" => (h1, H1) {
        global_attrs();
        content();
    }
}


//------------ main ----------------------------------------------------------

html_element!{
    "main" => (main, Main) {
        global_attrs();
        content();
    }
}


//------------ nav -----------------------------------------------------------

html_element!{
    "nav" => (nav, Nav) {
        global_attrs();
        content();
    }
}


//------------ p -------------------------------------------------------------

html_element!{
    "p" => (p, P) {
        global_attrs();
        content();
    }
}


//------------ script --------------------------------------------------------

html_element!{
    "script" => (script, Script) {
        attr(src);
        global_attrs();
        content();
    }
}


//------------ tt ------------------------------------------------------------

html_element!{
    "tt" => (tt, Tt) {
        global_attrs();
        content();
    }
}


//------------ HtmlTag -------------------------------------------------------

pub struct HtmlTag;

impl Element for HtmlTag {
    fn start<W: fmt::Write>(&self, wr: &mut W) -> fmt::Result {
        wr.write_str("<!DOCTYPE html>\n<html")
    }

    fn end<W: fmt::Write>(&self, wr: &mut W) -> fmt::Result {
        wr.write_str("</html>")
    }
}

