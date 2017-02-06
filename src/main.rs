extern crate railsite;

use railsite::site::Site;

fn main() {
    let site = Site::new("127.0.0.1:8080").unwrap();
    let _guard = site.serve();
}
