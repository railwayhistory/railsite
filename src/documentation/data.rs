//! Documentation Data.

use std::collections::HashMap;
use std::fs::File;
use std::sync::mpsc;
use std::sync::Arc;
use ignore::{WalkBuilder, WalkState};
use ignore::types::TypesBuilder;
use pulldown_cmark::{Parser, Options, html};
use pulldown_cmark::escape::escape_html;
use raildata::load::report;
use raildata::load::read::Utf8Chars;
use raildata::load::report::{Failed, Report, Reporter, Stage };
use raildata::load::yaml::Loader;
use raildata::types::{IntoMarked, LanguageCode, Location};
use crate::config::Config;
use super::load;


//------------ Documentation -------------------------------------------------

#[derive(Clone, Debug, Default)]
pub struct Documentation {
    langs: HashMap<LanguageCode, LangDoc>,
}

#[derive(Clone, Debug, Default)]
struct LangDoc {
    pages: HashMap<Arc<str>, Page>,
    topics: HashMap<String, Topic>,
}

impl Documentation {
    pub fn load(config: &Config) -> Result<Self, Failed> {
        Self::_load(config).map_err(|err| {
            for item in err.iter() {
                println!("{}", item)
            }
            Failed
        })
    }

    fn _load(config: &Config) -> Result<Self, Report> {
        let report = Reporter::new();
        let pages = match Self::load_tree(config, report.clone()) {
            Ok(pages) => pages,
            Err(_) => {
                return Err(report.unwrap())
            }
        };
        let res = Self::load_pages(pages, report.clone());
        if !report.is_empty() {
            Err(report.unwrap())
        }
        else {
            Ok(res)
        }
    }

    fn load_tree(
        config: &Config, report: Reporter
    ) -> Result<Vec<load::Page>, Failed> {
        let walk = WalkBuilder::new(&config.documentation).types(
            TypesBuilder::new()
                .add_defaults()
                .select("yaml")
                .build()
                .unwrap()
        ).build_parallel();
        
        let rx = {
            let (tx, rx) = mpsc::channel();
            walk.run(|| {
                let tx = tx.clone();
                let report = report.clone();
                Box::new(move |path| {
                    if let Ok(path) = path {
                        if let Some(file_type) = path.file_type() {
                            if file_type.is_dir() {
                                return WalkState::Continue
                            }
                        }
                        let path = report::Path::new(path.path());
                        match File::open(&path) {
                            Ok(file) => {
                                let mut report = report.clone()
                                    .stage(Stage::Translate)
                                    .with_path(path);
                                let res = {
                                    let mut loader = Loader::new(|v| {
                                        if let Ok(doc) = load::Page::from_yaml(
                                            v, &mut report
                                        ) {
                                            tx.send(doc).unwrap();
                                        }
                                    });
                                    loader.load(Utf8Chars::new(file))
                                };
                                if let Err(err) = res {
                                    let mut report = report.restage(
                                        Stage::Parse
                                    );
                                    report.error(err.marked(Location::NONE));
                                }
                            }
                            Err(err) => {
                                report.clone().stage(
                                    Stage::Parse
                                ).with_path(path).error(
                                    err.marked(Location::NONE)
                                )
                            }
                        }
                    }
                    WalkState::Continue
                })
            });
            rx
        };

        if report.is_empty() {
            Ok(rx.into_iter().collect())
        }
        else {
            Err(Failed)
        }
    }

    fn load_pages(pages: Vec<load::Page>, report: Reporter) -> Self {
        let mut report = report.stage(Stage::Translate);
        let mut res = Self::default();

        for page in pages {
            let path: Arc<str> = page.path.into_value().into_boxed_str().into();
            let langcode = page.lang.to_value();
            let lang = res.langs.entry(langcode).or_default();
            let mut content = String::new();

            for topic in page.topics {
                if let Some(key) = topic.key.as_ref() {
                    if lang.topics.insert(
                        key.as_value().clone(),
                        Topic {
                            path: path.clone(),
                        }
                    ).is_some() {
                        report.error_at(
                            page.origin.at(key.location()),
                            format!("duplicate topic '{}' in language {}",
                                key.as_value(), langcode
                            )
                        );
                    }

                    content.push_str("<div class=\"doc-topic\" id=\"");
                    escape_html(&mut content, key).unwrap();
                    content.push_str(">");
                    Self::render_content(
                        &mut content, topic.content.as_value()
                    );
                    content.push_str("</div>");
                }
                else {
                    Self::render_content(
                        &mut content, topic.content.as_value()
                    );
                }
            }
            lang.pages.insert(path, Page {
                title: page.title.into_value(),
                content
            });
        }

        res
    }

    fn render_content(target: &mut String, content: &str) {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        let parser = Parser::new_ext(content, options);
        html::push_html(target, parser);
    }

    pub fn get_topic_path(
        &self, lang: LanguageCode, key: &str
    ) -> Option<&str> {
        self.langs.get(&lang)
            .and_then(|doc| doc.topics.get(key))
            .map(|topic| topic.path.as_ref())
    }

    pub fn get_page(
        &self, lang: LanguageCode, path: &str
    ) -> Option<&Page> {
        self.langs.get(&lang).and_then(|doc| {
            doc.pages.get(path)
        })
    }
}


//------------ Page ----------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Page {
    pub title: String,
    pub content: String,
}


//------------ Topic ---------------------------------------------------------

/// An entry in the topic index.
#[derive(Clone, Debug)]
pub struct Topic {
    path: Arc<str>,
}

