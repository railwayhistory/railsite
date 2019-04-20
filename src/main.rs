extern crate actix_web;
extern crate raildata;
extern crate railsite;

use std::env;
use actix_web::server;
//use raildata::load::load_tree;
use raildata::library::Library;
use railsite::Railsite;


fn main() {
    let path = env::args().nth(1).unwrap_or("test-data/output.bin".into());
    /*
    let library = match load_tree(Path::new(&path).into()) {
        Ok(library) => library,
        Err(mut err) => {
            err.sort();
            println!("{} errors.", err.len());
            for item in err.iter() {
                println!("{}", item)
            }
            ::std::process::exit(1);
        }
    };
    */
    let library = Library::read(std::fs::File::open(path).unwrap()).unwrap();
    println!("Ready!");

    server::new(move || Railsite::new(library.clone()))
        .bind("127.0.0.1:8080")
        .unwrap()
        .run();
}

