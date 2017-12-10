extern crate futures;
extern crate hyper;
extern crate raildata;
/*
extern crate url;
*/
extern crate xml;

pub use self::site::Railsite;

mod errors;
pub mod htmlfn;
mod site;
mod statics;
mod views;

