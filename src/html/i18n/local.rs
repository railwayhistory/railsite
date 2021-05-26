//! Localization of terms related to locality and languages.

use raildata::types::local::LocalCode;
use crate::html::target::{RenderText, Text};
use crate::i18n::Lang::*;

impl RenderText for LocalCode {
    fn render(self, target: &mut Text) {
        let s = match target.lang() {
            En => match self.try_into_language() {
                Ok(lang) => self::en::lang_noun(lang),
                Err(country) => self::en::country_noun(country),
            },
            De => match self.try_into_language() {
                Ok(lang) => self::de::lang_noun(lang),
                Err(country) => self::de::country_noun(country),
            }
        };
        match s {
            Some(s) => target.push_str(s),
            None => target.push_str(self.as_str()),
        }
    }
}

pub mod en {
    use raildata::types::local::{
        LanguageCode as L,
        CountryCode as C,
        LocalCode
    };

    pub fn lang_noun(lang: L) -> Option<&'static str> {
        match lang {
            L::CES => Some("Czech"),
            L::DAN => Some("Danish"),
            L::DEU => Some("German"),
            L::ENG => Some("English"),
            L::FRA => Some("French"),
            L::LAV => Some("Latvian"),
            L::NOB => Some("Norwegian"),
            L::NLD => Some("Dutch"),
            L::NNO => Some("Norwegian"),
            L::POL => Some("Polish"),
            L::RUS => Some("Russian"),
            L::SWE => Some("Swedish"),
            _ => None
        }
    }

    pub fn country_noun(country: C) -> Option<&'static str> {
        match country {
            C::AT => Some("Austria"),
            C::BE => Some("Belgium"),
            C::CH => Some("Switzerland"),
            C::DD => Some("East Germany"),
            C::DE => Some("Germany"),
            C::DK => Some("Denmark"),
            C::FR => Some("France"),
            C::GB => Some("Great Britain"),
            C::LT => Some("Latvia"),
            C::LU => Some("Luxembourg"),
            C::NL => Some("Netherlands"),
            C::NO => Some("Norway"),
            C::PL => Some("Polamd"),
            C::RU => Some("Russia"),
            C::SE => Some("Sweden"),
            _ => None
        }
    }

    pub fn lang_adjective(lang: L) -> Option<&'static str> {
        match lang {
            L::CES => Some("Czech"),
            L::DAN => Some("Danish"),
            L::DEU => Some("German"),
            L::ENG => Some("English"),
            L::FRA => Some("French"),
            L::LAV => Some("Latvian"),
            L::NOB => Some("Norwegian"),
            L::NLD => Some("Dutch"),
            L::NNO => Some("Norwegian"),
            L::POL => Some("Polish"),
            L::RUS => Some("Russian"),
            L::SWE => Some("Swedish"),
            _ => None
        }
    }

    pub fn country_adjective(country: C) -> Option<&'static str> {
        match country {
            C::AT => Some("Austrian"),
            C::BE => Some("Belgian"),
            C::CH => Some("Swiss"),
            C::DD => Some("East-german"),
            C::DE => Some("German"),
            C::DK => Some("Danish"),
            C::FR => Some("French"),
            C::GB => Some("British"),
            C::LT => Some("Latvian"),
            C::LU => Some("Luxembourgish"),
            C::NL => Some("Dutch"),
            C::NO => Some("Norwegian"),
            C::PL => Some("Polish"),
            C::RU => Some("Russian"),
            C::SE => Some("Swedish"),
            _ => None
        }
    }

    pub fn local_adjective(code: LocalCode) -> Option<&'static str> {
        match code.try_into_language() {
            Ok(lang) => lang_adjective(lang),
            Err(country) => country_adjective(country)
        }
    }
}

pub mod de {
    use raildata::types::local::{
        LanguageCode as L,
        CountryCode as C,
        LocalCode
    };

    pub fn lang_noun(lang: L) -> Option<&'static str> {
        match lang {
            L::CES => Some("Tschechisch"),
            L::DAN => Some("Dänisch"),
            L::DEU => Some("Deutsch"),
            L::ENG => Some("Englisch"),
            L::FRA => Some("Französisch"),
            L::LAV => Some("Litauisch"),
            L::NOB => Some("Norwegisch"),
            L::NLD => Some("Niederländisch"),
            L::NNO => Some("Norwegisch"),
            L::POL => Some("Polnisch"),
            L::RUS => Some("Russisch"),
            L::SWE => Some("Schwedisch"),
            _ => None
        }
    }

    pub fn country_noun(code: C) -> Option<&'static str> {
        match code {
            C::AT => Some("Österreich"),
            C::BE => Some("Belgien"),
            C::CH => Some("Schweiz"),
            C::DD => Some("DDR"),
            C::DE => Some("Deutschland"),
            C::DK => Some("Dänemark"),
            C::FR => Some("Frankreich"),
            C::GB => Some("Großbritannien"),
            C::LT => Some("Litauen"),
            C::LU => Some("Luxemburg"),
            C::NL => Some("Niederlande"),
            C::NO => Some("Norwegen"),
            C::PL => Some("Polen"),
            C::RU => Some("Russland"),
            C::SE => Some("Schweden"),
            _ => None
        }
    }

    pub fn masculine_lang_adjective(lang: L) -> Option<&'static str> {
        match lang {
            L::CES => Some("tschechischer"),
            L::DAN => Some("dänischer"),
            L::DEU => Some("deutscher"),
            L::ENG => Some("englischer"),
            L::FRA => Some("französischer"),
            L::LAV => Some("litauischer"),
            L::NOB => Some("norwegischer"),
            L::NLD => Some("niederländischer"),
            L::NNO => Some("norwegischer"),
            L::POL => Some("polnischer"),
            L::RUS => Some("russischer"),
            L::SWE => Some("schwedischer"),
            _ => None
        }
    }

    pub fn masculine_country_adjective(code: C) -> Option<&'static str> {
        match code {
            C::AT => Some("österreichischer"),
            C::BE => Some("belgischer"),
            C::CH => Some("schweizerischer"),
            C::DD => Some("deutscher"),
            C::DE => Some("deutscher"),
            C::DK => Some("dänischer"),
            C::FR => Some("französischer"),
            C::GB => Some("britischer"),
            C::LT => Some("litauischer"),
            C::LU => Some("luxemburger"),
            C::NL => Some("niederländischer"),
            C::NO => Some("norwegischer"),
            C::PL => Some("polnischer"),
            C::RU => Some("russischer"),
            C::SE => Some("schwedischer"),
            _ => None
        }
    }

    pub fn masculine_local_adjective(code: LocalCode) -> Option<&'static str> {
        match code.try_into_language() {
            Ok(lang) => masculine_lang_adjective(lang),
            Err(country) => masculine_country_adjective(country)
        }
    }

    pub fn feminine_lang_adjective(lang: L) -> Option<&'static str> {
        match lang {
            L::CES => Some("tschechische"),
            L::DAN => Some("dänische"),
            L::DEU => Some("deutsche"),
            L::ENG => Some("englische"),
            L::FRA => Some("französische"),
            L::LAV => Some("litauische"),
            L::NOB => Some("norwegische"),
            L::NLD => Some("niederländische"),
            L::NNO => Some("norwegische"),
            L::POL => Some("polnische"),
            L::RUS => Some("russische"),
            L::SWE => Some("schwedische"),
            _ => None
        }
    }

    pub fn feminine_country_adjective(code: C) -> Option<&'static str> {
        match code {
            C::AT => Some("österreichische"),
            C::BE => Some("belgische"),
            C::CH => Some("schweizerische"),
            C::DD => Some("deutsche"),
            C::DE => Some("deutsche"),
            C::DK => Some("dänische"),
            C::FR => Some("französische"),
            C::GB => Some("britische"),
            C::LT => Some("litauische"),
            C::LU => Some("luxemburge"),
            C::NL => Some("niederländische"),
            C::NO => Some("norwegische"),
            C::PL => Some("polnische"),
            C::RU => Some("russische"),
            C::SE => Some("schwedische"),
            _ => None
        }
    }

    pub fn feminine_local_adjective(code: LocalCode) -> Option<&'static str> {
        match code.try_into_language() {
            Ok(lang) => feminine_lang_adjective(lang),
            Err(country) => feminine_country_adjective(country)
        }
    }
}

