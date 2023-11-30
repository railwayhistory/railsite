//! Build script that generates the i18n module.

use std::{env, fmt, fs};
use std::collections::HashMap;
use std::path::Path;

const LANG: &str = "crate::lang::Lang";
//const LANG_VARIANTS: &[&str] = &["en", "de"];
const REQUEST_STATE: &str = "crate::state::RequestState";

//------------ main ----------------------------------------------------------

fn main() {
    let mut target = String::new();

    terms(&mut target);

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("i18n.rs");
    fs::write(&dest_path, &target).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=i18n/terms.yaml");
}


//------------ terms ---------------------------------------------------------

type Terms = HashMap<String, HashMap<String, String>>;

#[derive(Default)]
struct Module {
    mods: HashMap<String, Box<Self>>,
    funcs: HashMap<String, HashMap<String, String>>,
}

fn terms(target: &mut String) {
    let input = serde_yaml::from_str::<Terms>(
        &fs::read_to_string("i18n/terms.yaml").unwrap()
    ).unwrap();

    let mut output = Module::default();
    for (ident, content) in input {
        let mut ident = ident.split("::").collect::<Vec<_>>();
        let func = ident.pop().unwrap();
        let mut module = &mut output;
        let mut ident = ident.into_iter();
        while let Some(word) = ident.next() {
            module = module.mods.entry(word.into()).or_insert_with(|| {
                Module::default().into()
            })
        }
        module.funcs.insert(func.into(), content);
    }

    writeln!(target, "pub mod term {{");
    term_module(&output, target);
    writeln!(target, "}}");
}

fn term_module(module: &Module, target: &mut String) {
    for (name, module) in &module.mods {
        writeln!(target, "pub mod {} {{", name);
        term_module(module, target);
        writeln!(target, "}}");
    }

    for (name, content) in &module.funcs {
        writeln!(target,
            "pub fn {}(\
                 state: &{}\
             ) -> &'static str {{\
                 match state.lang() {{",
             name, REQUEST_STATE
        );
        for (lang, term) in content {
            writeln!(target,
                "{}::{}{} => \"{}\",",
                LANG, &lang[0..1].to_uppercase(), &lang[1..], term
            );
        }
        writeln!(target, "    }}\n}}");
    }
}


//------------ WriteOrPanic --------------------------------------------------

/// A target for writing formatted data into without error.
///
/// This provides a method `write_fmt` for use with the `write!` macro and
/// friends that does not return a result. Rather, it panics if an error
/// occurs.
pub trait WriteOrPanic {
    fn write_fmt(&mut self, args: fmt::Arguments);
}

impl WriteOrPanic for String {
    fn write_fmt(&mut self, args: fmt::Arguments) {
        std::fmt::Write::write_fmt(self, args).expect("formatting failed");
    }
}


