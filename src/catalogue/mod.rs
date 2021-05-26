
pub mod document;
pub mod organization;
pub mod point;
pub mod source;


use raildata::document::DocumentLink;
use raildata::library::{Library, LinkTarget};


//------------ Catalogue -----------------------------------------------------

/// The meta-information we keep for finding documents.
#[derive(Clone, Default)]
pub struct Catalogue {
    pub names: self::document::NameIndex,
    pub countries: self::organization::CountryIndex,
    pub doc_nums: self::document::DocumentNumbers,
    pub country_lines: self::organization::CountryLines,
    pub property_lines: self::organization::PropertyLines,
    pub point_connections: self::point::PointConnections,
    pub source: self::source::SourceRefs,
}

impl Catalogue {
    pub fn new(library: &Library) -> Self {
        let mut res = Self::default();
        library.links().for_each(|link| res.process_link(link, library));
        res.finalize(library);
        res
    }

    fn process_link(&mut self, link: DocumentLink, library: &Library) {
        let document = library.resolve(link);

        self.names.insert(document, link);
        self.countries.insert(document, link);
        self.doc_nums.insert(document);
        self.country_lines.insert(document, link, library);
        self.property_lines.insert(document, link);
        self.point_connections.insert(document, link);
        self.source.insert(document, link, library);
    }

    fn finalize(&mut self, library: &Library) {
        self.names.finalize(library);
        self.countries.finalize(library);
        self.doc_nums.finalize(library);
        self.country_lines.finalize(library);
        self.property_lines.finalize(library);
        self.point_connections.finalize(library);
        self.source.finalize(library);
    }
}

