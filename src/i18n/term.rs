use crate::lang::Lang;
use crate::state::RequestState;


pub mod nav {
    use super::*;

    pub fn home(state: &RequestState) -> &'static str {
        match state.lang() {
            Lang::En => "Home",
            Lang::De => "Start",
        }
    }

    pub fn browse(state: &RequestState) -> &'static str {
        match state.lang() {
            Lang::En => "Browse",
            Lang::De => "Stöbern",
        }
    }

    pub fn map(state: &RequestState) -> &'static str {
        match state.lang() {
            Lang::En => "Map",
            Lang::De => "Karte",
        }
    }

    pub fn documentation(state: &RequestState) -> &'static str {
        match state.lang() {
            Lang::En => "Documentation",
            Lang::De => "Dokumentation",
        }
    }

    pub fn toggle_nav_bar(state: &RequestState) -> &'static str {
        match state.lang() {
            Lang::En => "Toggle naviation bar",
            Lang::De => "Navigations ein- oder ausblenden",
        }
    }
}

