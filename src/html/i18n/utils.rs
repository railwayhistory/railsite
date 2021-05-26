use crate::i18n::Lang::*;
use super::super::target::{Content, RenderText, Text};


//------------ join ----------------------------------------------------------

pub fn join<I, T, F>(cont: &mut Content, mut iter: I, render: F)
where I: Iterator<Item = T>, F: Fn(&mut Content, T) {
    match iter.next() {
        Some(item) => render(cont, item),
        None => return
    }

    for item in iter {
        cont.text(", ");
        render(cont, item);
    }
}


//------------ and_join ------------------------------------------------------

pub fn and_join<I, T, F>(cont: &mut Content, iter: I, render: F)
where I: Iterator<Item = T>, F: Fn(&mut Content, T) {
    match cont.lang() {
        En => and_join_en(cont, iter, render),
        De => and_join_de(cont, iter, render)
    }
}

fn and_join_en<I, T, F>(cont: &mut Content, mut iter: I, render: F)
where I: Iterator<Item = T>, F: Fn(&mut Content, T) {
    match iter.next() {
        Some(item) => render(cont, item),
        None => return
    }

    let second = match iter.next() {
        Some(item) => item,
        None => return
    };

    let mut prev = match iter.next() {
        Some(item) => item,
        None => {
            cont.text(" and ");
            render(cont, second);
            return
        }
    };

    while let Some(item) = iter.next() {
        cont.text(", ");
        render(cont, prev);
        prev = item;
    }

    cont.text(", and ");
    render(cont, prev);
}

fn and_join_de<I, T, F>(cont: &mut Content, mut iter: I, render: F)
where I: Iterator<Item = T>, F: Fn(&mut Content, T) {
    match iter.next() {
        Some(item) => render(cont, item),
        None => return
    }

    let mut prev = match iter.next() {
        Some(item) => item,
        None => return
    };

    while let Some(item) = iter.next() {
        cont.text(", ");
        render(cont, prev);
        prev = item;
    }

    cont.text(" und ");
    render(cont, prev);
}


//------------ capitalize ----------------------------------------------------

pub fn capitalize(s: &str) -> Capitalize {
    Capitalize(s)
}

pub struct Capitalize<'a>(&'a str);

impl<'a> RenderText for Capitalize<'a> {
    fn render(self, target: &mut Text) {
        let first = match self.0.chars().next() {
            Some(first) => first,
            None => return
        };
        target.write_fmt(format_args!("{}", first.to_uppercase()));
        target.push_str(&self.0[first.len_utf8()..]);
    }
}

