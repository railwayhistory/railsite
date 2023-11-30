use htmlfn::core::Content;
use htmlfn::core::display;
use raildata::document::line::data::{
    Category, Goods, Electrified, Passenger, Status
};
use crate::i18n;
use crate::state::RequestState;

pub fn category(cat: Category, state: &RequestState) -> &'static str {
    use self::Category::*;

    match cat {
        DeHauptbahn => i18n::term::line::category::de_hauptbahn(state),
        DeNebenbahn => i18n::term::line::category::de_nebenbahn(state),
        DeKleinbahn => i18n::term::line::category::de_kleinbahn(state),
        DeAnschl => i18n::term::line::category::de_anschl(state),
        DeBfgleis => i18n::term::line::category::de_bfgleis(state),
        DeStrab => i18n::term::line::category::de_strab(state),
        GbLight => i18n::term::line::category::gb_light(state),
        _ => cat.short_str(),
    }
}

pub fn electrified<'a>(
    el: &'a Electrified,  state: &RequestState
) -> impl Content + 'a {
    use raildata::document::line::data::ElSystem::*;
    use raildata::document::line::data::AcDc::*;

    (
        el.generic().map(|gen| {
            Some((
                match gen.system {
                    Ole => i18n::term::line::electrified::ole(state),
                    Rail => i18n::term::line::electrified::rail(state),
                    Rail4 => i18n::term::line::electrified::rail4(state),
                },
                " ",
                display(gen.voltage),
                "\u{202f}V ",
                match gen.frequency {
                    Ac16 => i18n::term::line::electrified::ac16(state),
                    Ac25 => i18n::term::line::electrified::ac25(state),
                    Ac50 => i18n::term::line::electrified::ac50(state),
                    Tc50 => i18n::term::line::electrified::tc50(state),
                    Dc => i18n::term::line::electrified::dc(state),
                }
            ))
        }),
        el.generic().is_none().then(|| {
            i18n::term::line::electrified::none(state)
        })
    )
}

pub fn goods(goods: Goods, state: &RequestState) -> &'static str {
    use self::Goods::*;

    match goods {
        None => i18n::term::line::goods::none(state),
        Limited => i18n::term::line::goods::limited(state),
        Full => i18n::term::line::goods::full(state),
    }
}

pub fn passenger(value: Passenger, state: &RequestState) -> &'static str {
    use self::Passenger::*;

    match value {
        None => i18n::term::line::passenger::none(state),
        Limited => i18n::term::line::passenger::limited(state),
        Historic => i18n::term::line::passenger::historic(state),
        Seasonal => i18n::term::line::passenger::seasonal(state),
        Tourist => i18n::term::line::passenger::tourist(state),
        Full => i18n::term::line::passenger::full(state),
    }
}

pub fn status(value: Status, state: &RequestState) -> &'static str {
    use self::Status::*;

    match value {
        None => i18n::term::line::status::none(state),
        Planned => i18n::term::line::status::planned(state),
        Construction => i18n::term::line::status::construction(state),
        Open => i18n::term::line::status::open(state),
        Suspended => i18n::term::line::status::suspended(state),
        Reopened => i18n::term::line::status::open(state),
        Closed => i18n::term::line::status::closed(state),
        Removed => i18n::term::line::status::removed(state),
        Released => i18n::term::line::status::released(state),
    }
}

