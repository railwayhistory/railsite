//! Information concering all documents.

use radix_trie::{Trie, TrieCommon};
use raildata::document::{Document, DocumentLink};
use raildata::library::Library;
use raildata::types::List;
use unicode_normalization::UnicodeNormalization;

//------------ NameIndex -----------------------------------------------------

#[derive(Clone, Debug, Default)]
pub struct NameIndex {
    names: Trie<String, List<(String, DocumentLink)>>,
}

impl NameIndex {
    pub(super) fn insert(
        &mut self, document: &Document, link: DocumentLink
    ) {
        self.insert_name(document.key().to_string(), link);
        document.process_names(|name| self.insert_name(name, link))
    }

    fn insert_name(&mut self, name: String, link: DocumentLink) {
        let term = Self::normalize_name(&name);
        if let Some(value) = self.names.get_mut(&term) {
            value.push((name, link))
        }
        else {
            self.names.insert(term, List::with_value((name, link)));
        }
    }

    fn normalize_name(name: &str) -> String {
        name.nfd()
            .filter(|ch| ch.is_alphanumeric())
            .flat_map(|ch| ch.to_lowercase())
            .collect()
    }

    pub(super) fn finalize(&mut self, _: &Library) {
    }

    pub fn search(
        &self, prefix: &str, count: usize
    ) -> impl Iterator<Item = (&str, DocumentLink)> {
        let prefix = Self::normalize_name(prefix);
        self.names.get_raw_ancestor(&prefix).iter()
            .filter(move |(key, _)| key.starts_with(&prefix))
            .flat_map(|(_, value)| value)
            .map(|(name, link)| (name.as_str(), *link))
            .take(count)
    }
}


//------------ DocumentNumbers -----------------------------------------------

#[derive(Clone, Copy, Debug, Default)]
pub struct DocumentNumbers {
    pub total: usize,
    pub lines: usize,
    pub organizations: usize,
    pub paths: usize,
    pub points: usize,
    pub sources: usize,
    pub structures: usize,
}

impl DocumentNumbers {
    pub(super) fn insert(
        &mut self, document: &Document
    ) {
        self.total += 1;
        match *document {
            Document::Line(_) => self.lines += 1,
            Document::Organization(_) => self.organizations += 1,
            Document::Path(_) => self.paths += 1,
            Document::Point(_) => self.points += 1,
            Document::Source(_) => self.sources += 1,
            Document::Structure(_) => self.structures += 1,
        }
    }

    pub(super) fn finalize(&mut self, _: &Library) {
    }
}

