//! Translations for error pages.

use crate::i18n::Lang;
use crate::html::target::Content;

str_terms! {
    term_404_not_found {
        En => "404 Not Found",
        De => "404 Nicht Gefunden"
    }
    term_405_not_allowed {
        En => "405 Method Not Allowed",
        De => "405 Methode nicht erlaubt"
    }
}

pub fn path_not_found(lang: Lang, path: &str, cont: &mut Content) {
    match lang {
        Lang::En => {
            cont.text("The path ");
            cont.tt().text(path);
            cont.text(" could not be found on the server.");
        }
        Lang::De => {
            cont.text("Der Pfad ");
            cont.tt().text(path);
            cont.text(" konnte auf dem Server nicht gefunden werden.");
        }
    }
}

pub fn method_not_allowed(
    lang: Lang, method: &str, path: &str, cont: &mut Content
) {
    match lang {
        Lang::En => {
            cont.text("The method ");
            cont.tt().text(method);
            cont.text(" is not allowed for path ");
            cont.tt().text(path);
            cont.text(".");
        }
        Lang::De => {
            cont.text("Die Methode ");
            cont.tt().text(method);
            cont.text(" ist für den Pfad ");
            cont.tt().text(path);
            cont.text(" nicht erlaubt.");
        }
    }
}

