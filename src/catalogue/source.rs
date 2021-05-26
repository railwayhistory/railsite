//! Catalogue items for sources.

use std::collections::HashMap;
use raildata::document::{
    Document, DocumentLink, OrganizationLink, SourceLink
};
use raildata::library::Library;
use raildata::types::{List, Set};


//------------ SourceRefs ----------------------------------------------------

/// References for sources.
#[derive(Clone, Default)]
pub struct SourceRefs {
    /// The sources a person authored.
    pub creators: HashMap<OrganizationLink, List<(SourceLink, CreatorRoles)>>,

    /// The items in a collection.
    pub collection_items: HashMap<SourceLink, List<SourceLink>>,

    /// A source’s complete also set.
    pub also: HashMap<SourceLink, Set<SourceLink>>,

    /// The regards source attribute reversed.
    pub regards: HashMap<DocumentLink, List<SourceLink>>,
}

impl SourceRefs {
    pub(super) fn insert(
        &mut self,
        document: &Document,
        link: DocumentLink,
        library: &Library,
    ) {
        if let Some(source) = document.try_as_source() {
            let link = link.into();

            let mut creators: HashMap<_, CreatorRoles> = HashMap::new();
            for item in &source.author {
                creators.entry(
                    item.into_value()
                ).or_default().author = true;
            }
            for item in &source.editor {
                creators.entry(
                    item.into_value()
                ).or_default().editor = true;
            }
            for item in &source.organization {
                creators.entry(
                    item.into_value()
                ).or_default().organization = true;
            }
            for item in &source.publisher {
                creators.entry(
                    item.into_value()
                ).or_default().publisher = true;
            }

            for (org_link, roles) in creators {
                self.creators.entry(org_link).or_default().insert_sorted_by(
                    (link, roles),
                    |left, right| {
                        left.0.follow(library).date.sort_cmp(
                            &right.0.follow(library).date
                        )
                    }
                )
            }

            if let Some(collection) = source.collection {
                self.collection_items.entry(
                    collection.into_value()
                ).or_default().insert_sorted_by(link, |left, right| {
                    left.follow(library).date.sort_cmp(
                        &right.follow(library).date
                    )
                })
            }

            for also in &source.also {
                let also = also.into_value();
                self.also.entry(link).or_default().insert(also);
                self.also.entry(also).or_default().insert(link);
            }

            for doc in &source.regards {
                self.regards.entry(
                    doc.into_value()
                ).or_default().insert_sorted_by(link, |left, right| {
                    left.follow(library).date.sort_cmp(
                        &right.follow(library).date
                    )
                })
            }
        }
    }

    pub(super) fn finalize(&mut self, _library: &Library) {
    }
}


//------------ CreatorRoles --------------------------------------------------

/// The roles an organization can have creation of a source.
#[derive(Clone, Copy, Debug, Default)]
pub struct CreatorRoles {
    pub author: bool,
    pub editor: bool,
    pub organization: bool,
    pub publisher: bool,
}

