use std::io;
use std::fs::File;
use std::path::Path;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use hyper::{Body, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use raildata::library::Library;
use railsite::http::Request;
use railsite::site;

fn load_library(path: impl AsRef<Path>) -> Result<Library, io::Error> {
    let file = File::open(path)?;
    Library::read(file).map_err(|err| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("{}", err)
        )
    })
}

async fn process(
    request: hyper::Request<Body>,
    library: Library,
    base: Arc<String>,
) -> Result<Response<Body>, Infallible> {
    Ok(site::process(Request::new(request, library, base)))
}

#[tokio::main]
async fn main() {
    let library = load_library("test-data/output.bin").unwrap();
    println!("Successfully loaded library.");

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let base = Arc::new(String::from("http://localhost:8080"));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(move |_conn| {
        let library = library.clone();
        let base = base.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |r| {
                process(r, library.clone(), base.clone())
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

