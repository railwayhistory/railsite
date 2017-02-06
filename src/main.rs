extern crate hyper;
extern crate railsite;

use hyper::server::Http;
use railsite::site::Site;

fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(Site)).unwrap();
    server.run().unwrap();
}
