/// Render the changes of a line event.

use horrorshow::TemplateBuffer;
use raildata::document::{common, line};
use crate::i18n::Lang;
use crate::site::Site;

pub fn render_line_event(
    site: &Site, event: &line::Event, tmpl: &mut TemplateBuffer
) {
    match site.lang() {
        Lang::De => render_de(site, event, tmpl),
        Lang::En => render_en(site, event, tmpl),
    }
}


//------------ German --------------------------------------------------------

fn render_de(
    site: &Site, event: &line::Event, tmpl: &mut TemplateBuffer
) {
    if let Some(treaty) = event.treaty.as_ref() {
        render_de_treaty(site, event, treaty, tmpl)
    }
    else if let Some(concession) = event.concession.as_ref() {
        render_de_concession(site, event, concession, tmpl)
    }
    else if let Some(contract) = event.contract.as_ref() {
        render_de_contract(site, event, contract, tmpl)
    }
    else {
        render_de_base(site, event, tmpl)
    }
}

fn render_de_treaty(
    _site: &Site, _event: &line::Event, _treaty: &common::Contract,
    _tmpl: &mut TemplateBuffer
) {
}

fn render_de_concession(
    _site: &Site, _event: &line::Event, _concession: &line::Concession,
    _tmpl: &mut TemplateBuffer
) {
}

fn render_de_contract(
    _site: &Site, _event: &line::Event, _contract: &common::Contract,
    _tmpl: &mut TemplateBuffer
) {
}

fn render_de_base(
    _site: &Site, _event: &line::Event, _tmpl: &mut TemplateBuffer
) {
}


//------------ English -------------------------------------------------------

fn render_en(
    _site: &Site, _event: &line::Event, _tmpl: &mut TemplateBuffer
) {
}

