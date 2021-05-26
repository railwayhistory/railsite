//! Translations of the footer content.

use crate::html::target::Content;
use crate::i18n::Lang;


pub fn footer_content(lang: Lang, cont: &mut Content) {
    match lang {
        Lang::En => {
            cont.text(
                "The Railway History Database is made available under the "
            );
            cont.a().href("https://creativecommons.org/licenses/by/4.0/")
                .text("Creative Commons Attribution 4.0 International")
            ;
            cont.text(
                " license except for geographical data which is published \
                under the "
            );
            cont.a().href("https://opendatacommons.org/licenses/odbl/")
                .content(|cont| {
                    cont.text(
                        "Open Data Commons Open Database License (ODbL)"
                    )
                });
            cont.text(
                ". Images and other artwork may be licensed differently."
            );
        }
        Lang::De => {
            cont.text(
                "Die Railway History Database ist lizensiert unter "
            );
            cont.a().href("https://creativecommons.org/licenses/by/4.0/")
                .content(|cont| {
                    cont.text(
                        "Creative Commons Attribution 4.0 International"
                    )
                });
            cont.text(
                ", Geodaten unter "
            );
            cont.a().href("https://opendatacommons.org/licenses/odbl/")
                .content(|cont| {
                    cont.text(
                        "Open Data Commons Open Database License (ODbL)"
                    )
                });
            cont.text(
                ". Bilder und andere Medien können abweichend lizensiert sein."
            );
        }
    }
}

