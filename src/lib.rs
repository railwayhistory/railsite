extern crate futures;
#[macro_use] extern crate htmlfn;
extern crate hyper;
extern crate raildata;
/*
extern crate url;
*/

pub use self::site::Railsite;

mod errors;
mod site;
mod statics;
mod views;

