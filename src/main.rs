extern crate hyper;
extern crate railsite;

use hyper::rt::Future;
use railsite::Railsite;


fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();
    let server = hyper::Server::bind(&addr)
        .serve(Railsite)
        .map_err(|e| eprintln!("server error: {}", e));
    hyper::rt::run(server);
}

