//! Translations related to source documents.

str_terms! {
    term_part_overview {
        En => "Overview",
        De => "Übersicht",
    }

    term_overview_entry_type {
        En => "Entry type",
        De => "Literaturtyp",
    }
    term_overview_author {
        En => "Author",
        De => "Author",
    }
    term_overview_authors {
        En => "Authors",
        De => "Authoren",
    }
    term_overview_collection {
        En => "In",
        De => "In",
    }
    term_overview_date {
        En => "Published",
        De => "Erschienen",
    }
    term_overview_digital {
        En => "Digital version",
        De => "Digitalisat",
    }
    term_overview_edition {
        En => "Edition",
        De => "Ausgabe",
    }
    term_overview_editor {
        En => "Editor",
        De => "Herausgeber",
    }
    term_overview_editors {
        En => "Editors",
        De => "Herausgeber",
    }
    term_overview_isbn {
        En => "ISBN",
        De => "ISBN",
    }
    term_overview_number {
        En => "Number",
        De => "Nummer",
    }
    term_overview_organization {
        En => "Organization",
        De => "Organisation",
    }
    term_overview_organizations {
        En => "Organizations",
        De => "Organisationen",
    }
    term_overview_pages {
        En => "Pages",
        De => "Seite",
    }
    term_overview_publisher {
        En => "Publisher",
        De => "Verleger",
    }
    term_overview_publishers {
        En => "Publishers",
        De => "Verleger",
    }
    term_overview_revision {
        En => "Revision",
        De => "Bearbeitungsstand",
    }
    term_overview_title {
        En => "Title",
        De => "Titel",
    }
    term_overview_url {
        En => "URL",
        De => "URL",
    }
    term_overview_volume {
        En => "Volume",
        De => "Band",
    }
    term_overview_also {
        En => "Also published as",
        De => "Auch herausgegeben als",
    }
    term_overview_attribution {
        En => "Attribution",
        De => "Attribution",
    }
    term_overview_note {
        En => "Note",
        De => "Anmerkung",
    }
    term_overview_contents {
        En => "Contents",
        De => "Inhalt",
    }

    term_in_collection {
        En => "In: ",
        De => "In: ",
    }
}

lang_enum! {
    raildata::document::source::Subtype {
        En => {
            Article => "article",
            Book => "book",
            Inarticle => "in article",
            Issue => "issue",
            Journal => "journal",
            Map => "map",
            Online => "online",
            Series => "series",
            Volume => "volume",
            Misc => "miscellaneous",
        }
        De => {
            Article => "Artikel",
            Book => "Buch",
            Inarticle => "Artikelteil",
            Issue => "Ausgabe",
            Journal => "Zeitschrift",
            Map => "Karte",
            Online => "Online",
            Series => "Reihe",
            Volume => "Band",
            Misc => "Sonstige",
        }
    }
}

