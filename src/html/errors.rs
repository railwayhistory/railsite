//! Error pages.

use hyper::Method;
use crate::site::Site;
use super::{i18n, skeleton};
use super::target::{Content, Target, empty};


pub fn not_found(site: &Site, path: &str) -> Target {
    skeleton::headline(
        site,
        i18n::errors::term_404_not_found,
        empty,
        skeleton::Nav::Other,
        |cont: &mut Content| {
            cont.h1().text(i18n::errors::term_404_not_found)
        },
        |cont: &mut Content| {
            i18n::errors::path_not_found(site.lang(), path, cont);
        },
        empty
    )
}


pub fn method_not_allowed(site: &Site, method: &Method, path: &str) -> Target {
    skeleton::headline(
        site,
        i18n::errors::term_405_not_allowed,
        empty,
        skeleton::Nav::Other,
        |cont: &mut Content| {
            cont.h1().text(i18n::errors::term_405_not_allowed)
        },
        |cont: &mut Content| {
            i18n::errors::method_not_allowed(
                site.lang(), method.as_str(), path, cont
            );
        },
        empty
    )
}

