extern crate hyper;
extern crate railsite;

use hyper::server::Http;
use railsite::Railsite;


fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(Railsite)).unwrap();
    server.run().unwrap();
}

