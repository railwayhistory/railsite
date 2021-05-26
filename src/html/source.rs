use raildata::document::{Source, SourceLink};
use raildata::document::source::Subtype;
use raildata::types::{List, Marked};
use crate::site::Site;
use crate::site::source::Part;
use super::{cmark, i18n, organization, skeleton};
use super::components::property_view;
use super::i18n::utils::and_join;
use super::target::{Content, Target, Text, empty};


//------------ details -------------------------------------------------------

pub fn details(site: &Site, source: &Source, part: Part) -> Target {
    skeleton::multipart_sheet(
        site,
        |title: &mut Text| short_title_text(title, source, site),
        empty, // head
        skeleton::Nav::Other,
        |cont| {
            cont.h1().text(|text: &mut Text| {
                short_title_text(text, source, site)
            })
        },
        |nav| {
            nav.item(
                site.link_source_part(source, Part::Overview),
                matches!(part, Part::Overview),
                |cont| cont.text(i18n::source::term_part_overview)
            );
        },
        |cont| {
            match part {
                Part::Overview => overview_part(cont, source, site),
            }
        },
        empty, // scripts
    )
}
//------------ overview_part -------------------------------------------------

fn overview_part(cont: &mut Content, source: &Source, site: &Site) {
    property_view(cont, |view| {
        // subtype
        view.property(
            |name| name.text(i18n::source::term_overview_entry_type),
            |values| values.value(|cont| {
                cont.text(source.subtype.into_value())
            }),
        );

        // author
        if !source.author.is_empty() {
            view.property(
                |name| {
                    name.text(
                        if source.author.len() == 1 {
                            i18n::source::term_overview_author
                        }
                        else {
                            i18n::source::term_overview_authors
                        }
                    )
                },
                |values| {
                    values.value(|cont| {
                        and_join(
                            cont,
                            source.author.iter(),
                            |cont, link| {
                                organization::link(
                                    site, link.into_value(), cont
                                )
                            }
                        )
                    })
                }
            )
        }

        // editor
        match source.editor.len() {
            0 => { }
            1 => {
                view.property(
                    |name| name.text(i18n::source::term_overview_editor),
                    |values| {
                        values.value(|cont| {
                            organization::link(
                                site,
                                source.editor.first().unwrap().into_value(),
                                cont
                            )
                        })
                    }
                )
            }
            _ => {
                view.property(
                    |name| name.text(i18n::source::term_overview_editors),
                    |values| {
                        values.value(|cont| {
                            and_join(
                                cont,
                                source.editor.iter(),
                                |cont, link| {
                                    organization::link(
                                        site, link.into_value(), cont
                                    )
                                }
                            )
                        })
                    }
                )
            }
        }

        // organization
        match source.organization.len() {
            0 => { }
            1 => {
                view.property(
                    |name| name.text(i18n::source::term_overview_organization),
                    |values| {
                        values.value(|cont| {
                            organization::link(
                                site,
                                source.organization
                                    .first().unwrap().into_value(),
                                cont
                            )
                        })
                    }
                )
            }
            _ => {
                view.property(
                    |name| name.text(i18n::source::term_overview_organizations),
                    |values| {
                        values.value(|cont| {
                            and_join(
                                cont,
                                source.organization.iter(),
                                |cont, link| {
                                    organization::link(
                                        site, link.into_value(), cont
                                    )
                                }
                            )
                        })
                    }
                )
            }
        }

        // title
        if let Some(title) = source.title.as_ref() {
            view.property(
                |name| name.text(i18n::source::term_overview_title),
                |values| values.value(|cont| cont.text(title.as_str()))
            )
        }

        // edition
        if let Some(edition) = source.edition.as_ref() {
            view.property(
                |name| name.text(i18n::source::term_overview_edition),
                |values| values.value(|cont| cont.text(edition.as_str()))
            )
        }

        // collection
        if let Some(collection) = source.collection {
            view.property(
                |name| name.text(i18n::source::term_overview_collection),
                |values| values.value(|cont| {
                    citation(cont, collection.into_value(), site)
                })
            )
        }

        // volume
        if let Some(volume) = source.volume.as_ref() {
            view.property(
                |name| name.text(i18n::source::term_overview_volume),
                |values| values.value(|cont| cont.text(volume.as_str()))
            )
        }

        // number
        if let Some(number) = source.number.as_ref() {
            view.property(
                |name| name.text(i18n::source::term_overview_number),
                |values| values.value(|cont| cont.text(number.as_str()))
            )
        }

        // pages
        if let Some(pages) = source.pages.as_ref() {
            view.property(
                |name| name.text(i18n::source::term_overview_pages),
                |values| values.value(|cont| cont.text(pages.as_str()))
            )
        }

        // revision
        if let Some(revision) = source.revision.as_ref() {
            view.property(
                |name| name.text(i18n::source::term_overview_revision),
                |values| values.value(|cont| cont.text(revision.as_str()))
            )
        }

        // publisher
        match source.publisher.len() {
            0 => { }
            1 => {
                view.property(
                    |name| name.text(i18n::source::term_overview_publisher),
                    |values| {
                        values.value(|cont| {
                            organization::link(
                                site,
                                source.publisher
                                    .first().unwrap().into_value(),
                                cont
                            )
                        })
                    }
                )
            }
            _ => {
                view.property(
                    |name| name.text(i18n::source::term_overview_publishers),
                    |values| {
                        values.value(|cont| {
                            and_join(
                                cont,
                                source.publisher.iter(),
                                |cont, link| {
                                    organization::link(
                                        site, link.into_value(), cont
                                    )
                                }
                            )
                        })
                    }
                )
            }
        }

        // date
        if let Some(date) = source.date(site.library()) {
            view.property(
                |name| name.text(i18n::source::term_overview_date),
                |values| values.value(|cont| cont.text(date))
            );
        }

        // attribution
        if let Some(attribution) = source.attribution.as_ref() {
            view.property(
                |name| name.text(i18n::source::term_overview_attribution),
                |values| values.value(|cont| cont.text(attribution.as_str()))
            );
        }

        // isbn
        if let Some(isbn) = source.isbn.as_ref() {
            view.property(
                |name| name.text(i18n::source::term_overview_isbn),
                |values| values.value(|cont| cont.text(isbn.as_str()))
            )
        }

        // url
        if let Some(url) = source.url.as_ref() {
            view.property(
                |name| name.text(i18n::source::term_overview_url),
                |values| values.value(|cont| {
                    cont.a().href(url.as_str()).text(url.as_str())
                })
            )
        }

        // digital
        if !source.digital.is_empty() {
            view.property(
                |name| name.text(i18n::source::term_overview_digital),
                |values| values.value(|cont| {
                    if source.digital.len() == 1 {
                        let digital = source.digital.first().unwrap();
                        cont.a().href(digital.as_str()).text(
                            digital.as_str()
                        )
                    }
                    else {
                        cont.ul().content(|cont| {
                            for digital in &source.digital {
                                cont.li().content(|cont| {
                                    cont.a().href(digital.as_str()).text(
                                        digital.as_str()
                                    )
                                })
                            }
                        })
                    }
                })
            );
        }

        // note
        if let Some(note) = source.note.as_ref().and_then(|note|
            note.for_language(view.lang().into())
        ) {
            view.property(
                |name| name.text(i18n::source::term_overview_note),
                |values| values.value(|cont| cmark::render(cont, note, site))
            );
        }

        // also
        if let Some(also) = site.catalogue().source.also.get(&source.link()) {
            view.property(
                |name| name.text(i18n::source::term_overview_also),
                |values| values.value(|cont| cont.ul().content(|cont| {
                    for &link in also.iter() {
                        cont.li().content(|cont| citation(cont, link, site))
                    }
                }))
            )
        }

        // contents
        if let Some(contents) = site.catalogue().source.collection_items.get(
            &source.link()
        ) {
            view.property(
                |name| name.text(i18n::source::term_overview_contents),
                |values| values.value(|cont| cont.ul().content(|cont| {
                    for &link in contents.iter() {
                        overview_contents_item(cont, source, link, site)
                    }
                }))
            );
        }
    })
}

fn overview_contents_item(
    cont: &mut Content, source: &Source, link: SourceLink, site: &Site
) {
    let item = link.follow(site.library());
    cont.li().content(|cont| {
        if let (Subtype::Journal, Some(volume))
            = (source.subtype.into_value(), item.volume.as_ref())
        {
            cont.a().href(site.link_source(item)).text(volume.as_str())
        }
        else if let (Subtype::Volume, Some(number))
            = (source.subtype.into_value(), item.number.as_ref())
        {
            cont.a().href(site.link_source(item)).text(number.as_str())
        }
        else {
            let people = citation_people(cont, item, site);
            if people {
                cont.text(" ");
            }
            cont.a().href(site.link_source(item)).text(|text: &mut Text| {
                let title = citation_title(text, item, site);
                if !people && !title {
                    text.push_str(
                        source.key().as_str()
                    )
                }
            })
        }
    })
}


//------------ SourceList ----------------------------------------------------

#[derive(Clone, Debug, Default)]
pub struct SourceList {
    prefix: &'static str,
    list: Vec<SourceLink>,
}

impl SourceList {
    pub fn new(prefix: &'static str) -> Self {
        SourceList {
            prefix,
            list: Default::default()
        }
    }

    pub fn add(&mut self, link: SourceLink) -> usize {
        for (idx, item) in self.list.iter().enumerate() {
            if *item == link {
                return idx + 1
            }
        }
        self.list.push(link);
        self.list.len()
    }

    pub fn list(
        &mut self, cont: &mut Content, list: &List<Marked<SourceLink>>
    ) {
        for link in list {
            let num = self.add(link.into_value());
            cont.a()
                .href(|text: &mut Text| {
                    write!(text, "#{}-{}", self.prefix, num)
                })
            .text(|text: &mut Text| write!(text, " [{}] ", num));
        }
    }

    pub fn render(&mut self, cont: &mut Content, site: &Site) {
        cont.dl().class("component-source-list").content(|cont| {
            for (num, link) in self.list.iter().enumerate() {
                let num = num + 1;
                cont.dt()
                    .id(|text: &mut Text| {
                        write!(text, "#{}-{}", self.prefix, num)
                    })
                .text(|text: &mut Text| write!(text, "[{}]", num));
                cont.dd().content(|cont| {
                    citation(cont, *link, site)
                })
            }
        })
    }
}


//------------ Components ---------------------------------------------------

pub fn short_title(cont: &mut Content, source: &Source, site: &Site) {
    cont.text(|text: &mut Text| short_title_text(text, source, site))
}

pub fn short_title_text(text: &mut Text, source: &Source, site: &Site) {
    if !citation_short_title(text, source, site) {
        text.push_str(source.key().as_str())
    }
}


//------------ citation ------------------------------------------------------

pub fn citation(cont: &mut Content, link: SourceLink, site: &Site) {
    let source = link.follow(site.library());

    if citation_people(cont, source, site) {
        cont.text(" ");
    }

    cont.a().href(site.link_source(source)).text(|text: &mut Text| {
        if !citation_title(text, source, site) {
            text.push_str(
                source.key().as_str()
            )
        }
    });

    if let Some(collection) = source.collection {
        let collection = collection.follow(site.library());
        cont.text(" ");
        cont.text(i18n::source::term_in_collection);
        cont.text(|text: &mut Text| {
            if !citation_title(text, collection, site) {
                text.push_str(
                    collection.key().as_str()
                )
            }
        })
    }
}


fn citation_people(cont: &mut Content, source: &Source, site: &Site) -> bool {
    let iter = if !source.author.is_empty() {
        source.author.iter()
    }
    else if !source.editor.is_empty() {
        source.editor.iter()
    }
    else if !source.organization.is_empty() {
        source.organization.iter()
    }
    else {
        return false
    };

    and_join(
        cont, iter,
        |cont, link| {
            organization::title(cont, link.into_value(), site);
        }
    );
    cont.text(".");
    true
}

fn citation_title(text: &mut Text, source: &Source, site: &Site) -> bool {
    if let Some(title) = source.title.as_ref() {
        let title = title.trim();
        text.push_str(title);
        if let Some(edition) = source.edition.as_ref() {
            text.push_str(" (");
            text.push_str(edition);
            text.push_str(").");
        }
        else {
            if !title.ends_with('.') {
                text.push('.');
            }
        }
        true
    }
    else if let Some(designation) = source.designation.as_ref() {
        let designation = designation.trim();
        text.push_str(designation);
        if !designation.ends_with('.') {
            text.push('.');
        }
        true
    }
    else if source.subtype.into_value() == Subtype::Issue {
        if let (Some(collection), Some(number))
            = (source.collection, source.number.as_ref())
        {
            let collection = collection.follow(site.library());
            if citation_title(text, collection, site) {
                text.push_str(" ");
                text.push_str(number);
                if !number.ends_with('.') {
                    text.push('.');
                }
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }
    else if source.subtype.into_value() == Subtype::Volume {
        if let (Some(collection), Some(volume))
            = (source.collection, source.volume.as_ref())
        {
            let collection = collection.follow(site.library());
            if citation_title(text, collection, site) {
                text.push_str(" ");
                text.push_str(volume);
                if !volume.ends_with('.') {
                    text.push('.');
                }
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }
    else {
        false
    }
}

fn citation_short_title(text: &mut Text, source: &Source, site: &Site) -> bool {
    if let Some(title) = source.short_title.as_ref() {
        text.push_str(title.as_value());
        true
    }
    else if source.subtype.into_value() == Subtype::Issue {
        if let (Some(collection), Some(number))
            = (source.collection, source.number.as_ref())
        {
            let collection = collection.follow(site.library());
            if citation_short_title(text, collection, site) {
                text.push_str(" ");
                text.push_str(number);
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }
    else if source.subtype.into_value() == Subtype::Volume {
        if let (Some(collection), Some(volume))
            = (source.collection, source.volume.as_ref())
        {
            let collection = collection.follow(site.library());
            if citation_short_title(text, collection, site) {
                text.push_str(" ");
                text.push_str(volume);
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }
    else if let Some(title) = source.title.as_ref() {
        text.push_str(title.as_str());
        true
    }
    else if let Some(designation) = source.designation.as_ref() {
        text.push_str(designation.as_str());
        true
    }
    else {
        false
    }
}

