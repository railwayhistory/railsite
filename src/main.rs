extern crate actix_web;
extern crate raildata;
extern crate railsite;

use std::env;
use std::path::Path;
use std::sync::Arc;
use actix_web::server;
use raildata::load::load_tree;
use railsite::Railsite;


fn main() {
    let path = env::args().nth(1).unwrap_or("test-data".into());
    let store = match load_tree(Path::new(&path).into()) {
        Ok(store) => store,
        Err(mut err) => {
            err.sort();
            println!("{} errors.", err.len());
            for item in err.iter() {
                println!("{}", item)
            }
            ::std::process::exit(1);
        }
    };
    let store = Arc::new(store);

    server::new(move || Railsite::new(store.clone()))
        .bind("127.0.0.1:8080")
        .unwrap()
        .run();
}

