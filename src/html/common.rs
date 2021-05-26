use raildata::document::common::{Agreement, Alternative, Basis};
use raildata::types::EventDate;
use crate::site::Site;
use super::i18n;
use super::source::SourceList;
use super::target::Content;


pub fn event_alternative(
    cont: &mut Content,
    alt: &Alternative,
    _site: &Site,
    sources: &mut SourceList,
) {
    i18n::common::alternative_base(alt, cont);
    sources.list(cont, &alt.document);
    sources.list(cont, &alt.source);
}

pub fn event_basis(
    cont: &mut Content,
    basis: &Basis,
    site: &Site,
    sources: &mut SourceList,
) {
    i18n::common::basis_base(cont, basis, site);
    sources.list(cont, &basis.document);
    sources.list(cont, &basis.source);
}


pub fn event_agreement(
    cont: &mut Content,
    agreement: &Agreement,
    date: &EventDate,
    site: &Site,
) {
    i18n::common::agreement_base(cont, agreement, date, site);
}

