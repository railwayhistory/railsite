//! The source of the documentation.
//!

use raildata::load::report::{Failed, Origin, PathReporter};
use raildata::load::yaml::{FromYaml, Value};
use raildata::types::{LanguageCode, Marked};


//------------ Page ----------------------------------------------------------

/// A page of topics.
#[derive(Clone, Debug)]
pub struct Page {
    /// The path under which the pages should be accessible.
    pub path: Marked<String>,

    /// The language of the content of the page.
    pub lang: Marked<LanguageCode>,

    /// The title of the page.
    pub title: Marked<String>,

    /// The topics on the page.
    pub topics: Vec<Topic>,

    /// The origin of page’s source.
    pub origin: Origin
}

impl Page {
    pub fn from_yaml(
        value: Value,
        report: &mut PathReporter,
    ) -> Result<Self, Failed> {
        let origin = Origin::new(report.path(), value.location());
        let mut doc = value.into_mapping(report)?;
        let path = doc.take("path", &(), report);
        let lang = doc.take("lang", &(), report);
        let title = doc.take("title", &(), report);
        let topics = doc.take("topics", &(), report);
        doc.exhausted(report)?;

        Ok(Page {
            path: path?,
            lang: lang?,
            title: title?,
            topics: topics?,
            origin
        })
    }
}


//------------ Topic ---------------------------------------------------------

/// A topic.
///
/// Topics are the base unit of documentation. They describe a single thing.
#[derive(Clone, Debug)]
pub struct Topic {
    /// An optional unique key identifying this particular topic.
    ///
    /// If the key is present, the topic can be linked to under this name.
    pub key: Option<Marked<String>>,

    /// The content.
    pub content: Marked<String>,
}

impl FromYaml<()> for Topic {
    fn from_yaml(
        value: Value,
        _: &(),
        report: &mut PathReporter,
    ) -> Result<Self, Failed> {
        let mut doc = value.into_mapping(report)?;
        let key = doc.take_opt("key", &(), report);
        let content = doc.take("content", &(), report);
        doc.exhausted(report)?;

        Ok(Topic {
            key: key?,
            content: content?,
        })
    }
}


