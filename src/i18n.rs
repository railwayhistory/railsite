
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
            Lang::De => "lang=de; Max-Age=2592000;  HttpOnly",
            Lang::En => "lang=en; Max-Age=2592000;  HttpOnly",
        }
    }
}

impl Default for Lang {
    fn default() -> Self {
        Lang::En
    }
}

