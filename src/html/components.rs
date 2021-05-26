//! Generic building blocks of the HTML output.

use crate::i18n::Lang;
use super::i18n;
use super::target::Content;


//------------ stub_badge ----------------------------------------------------

pub fn stub_badge(cont: &mut Content) {
    cont.div().class("component-stub-badge").text(
        i18n::components::term_stub_badge
    )
}

//------------ property_table ------------------------------------------------

pub fn property_table<F: FnOnce(PropertyTable)>(
    content: &mut Content, op: F
) {
    op(PropertyTable {
        content: content.table().class("component-property-table").get_content()
    })
}

/// A table listing properties.
pub struct PropertyTable<'a> {
    content: Content<'a>,
}

impl<'a> PropertyTable<'a> {
    pub fn property(
        &mut self,
        name: impl FnOnce(&mut Content),
        value: impl FnOnce(&mut Content)
    ) {
        self.content.tr().content(|cont| {
            cont.th().content(name);
            cont.td().content(value);
        });
    }
}


//------------ property_view -------------------------------------------------

pub fn property_view<F: FnOnce(&mut PropertyView)>(
    content: &mut Content, op: F
) {
    op(&mut PropertyView {
        content: content.div().class("component-properties").get_content()
    })
}

pub struct PropertyView<'a> {
    content: Content<'a>,
}

impl<'a> PropertyView<'a> {
    pub fn lang(&self) -> Lang {
        self.content.lang()
    }

    pub fn property(
        &mut self,
        name: impl FnOnce(&mut Content),
        values: impl FnOnce(&mut PropertyViewItem),
    ) {
        self.content.h3().content(name);
        values(&mut PropertyViewItem {
            content: {
                self.content.div().class(
                    "component-properties-values"
                ).get_content()
            }
        })
    }
}

pub struct PropertyViewItem<'a> {
    content: Content<'a>
}

impl<'a> PropertyViewItem<'a> {
    pub fn dated(
        &mut self,
        date: impl FnOnce(&mut Content),
        value: impl FnOnce(&mut Content)
    ) {
        self.content.p().class("component-properties-date").content(date);
        self.content.div().class("component-properties-value").content(value);
    }

    pub fn value(
        &mut self,
        value: impl FnOnce(&mut Content)
    ) {
        self.content.p().class("component-properties-date").text("");
        self.content.div().class("component-properties-value").content(value);
    }

    pub fn lang(&self) -> Lang {
        self.content.lang()
    }
}

