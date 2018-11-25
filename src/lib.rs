extern crate actix_web;
#[macro_use] extern crate htmlfn;
extern crate raildata;

pub use self::app::Railsite;

#[macro_use] mod statics;

mod app;
mod html;
mod views;

/*
mod core;
mod errors;
mod index;
mod site;
*/
