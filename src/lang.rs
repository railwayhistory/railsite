use htmlfn::core::{AttributeValue, Target};
//use raildata::types::date::{Date, Precision};
//use raildata::types::LanguageCode;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Lang {
    De,
    En
}

impl Lang {
    pub fn from_code(code: &str) -> Self {
        match code {
            "de" => Lang::De,
            _ => Lang::En,
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            Lang::De => "de",
            Lang::En => "en",
        }
    }

    pub fn cookie(self) -> &'static str {
        match self {
            Lang::De => "lang=de; path=/; Max-Age=2592000;  HttpOnly",
            Lang::En => "lang=en; path=/; Max-Age=2592000;  HttpOnly",
        }
    }

    pub fn all() -> impl Iterator<Item = Self> {
        [Lang::En, Lang::De].iter().map(|item| *item)
    }
}

impl Default for Lang {
    fn default() -> Self {
        Lang::En
    }
}

impl AsRef<str> for Lang {
    fn as_ref(&self) -> &str {
        self.code()
    }
}

impl AttributeValue for Lang {
    fn render_attr_value(self, target: &mut Target) {
        self.code().render_attr_value(target)
    }
}

/*
impl From<Lang> for LanguageCode {
    fn from(lang: Lang) -> LanguageCode {
        match lang {
            Lang::De => LanguageCode::DEU,
            Lang::En => LanguageCode::ENG,
        }
    }
}

impl Lang {
    pub fn render_date(self, date: &Date, tmpl: &mut TemplateBuffer) {
        match self {
            Lang::De => {
                match date.precision() {
                    Precision::Exact => { }
                    Precision::Circa => { write!(tmpl, "ca. "); }
                    Precision::Before => { write!(tmpl, "vor "); }
                    Precision::After => { write!(tmpl, "nach "); }
                }
                if let Some(day) = date.day() {
                    write!(tmpl, "{}. ", day);
                }
                if let Some(month) = date.month() {
                    write!(tmpl, "{}", roman(month));
                }
                write!(tmpl, "{}", date.year());
                if date.doubt() {
                    write!(tmpl, " ?");
                }
            }
            Lang::En => {
                match date.precision() {
                    Precision::Exact => { }
                    Precision::Circa => { write!(tmpl, "c. "); }
                    Precision::Before => { write!(tmpl, "before "); }
                    Precision::After => { write!(tmpl, "after "); }
                }
                write!(tmpl, "{:04}", date.year());
                if let Some(month) = date.month() {
                    write!(tmpl, "-{:02}", month);
                }
                if let Some(day) = date.day() {
                    write!(tmpl, "-{:02}", day);
                }
                if date.doubt() {
                    write!(tmpl, " ?");
                }
            }
        }
    }
}


fn roman(val: u8) -> &'static str {
    match val {
        1 => "I. ",
        2 => "II. ",
        3 => "III. ",
        4 => "IV. ",
        5 => "V. ",
        6 => "VI. ",
        7 => "VII. ",
        8 => "VIII. ",
        9 => "IX. ",
        10 => "X. ",
        11 => "XI. ",
        12 => "XII. ",
        _ => ""
    }
}
*/
