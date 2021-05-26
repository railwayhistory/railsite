//! Translations for index pages.

str_terms! {
    term_home_title {
        En => "The Railway History Database",
        De => "The Railway History Database",
    }
    term_home_country_lines {
        En => "Lines by Country",
        De => "Strecken nach Ländern",
    }
    term_home_statistics {
        En => "Statistics",
        De => "Statistik"
    }
}

lang_fn! {
    home_statistics_total_documents(count: usize) {
        En => { "{} documents", count }
        De => { "{} Einträge", count }
    }
    home_statistics_line_num(count: usize) {
        En => { "{} lines", count }
        De => { "{} Strecken", count }
    }
    home_statistics_point_num(count: usize) {
        En => { "{} points", count }
        De => { "{} Betriebsstellen", count }
    }
    home_statistics_structure_num(count: usize) {
        En => { "{} structures", count }
        De => { "{} Kunstbauten", count }
    }
    home_statistics_organization_num(count: usize) {
        En => { "{} organization", count }
        De => { "{} Organisationen", count }
    }
    home_statistics_source_num(count: usize) {
        En => { "{} source documents", count }
        De => { "{} Quelldokumente", count }
    }
}
