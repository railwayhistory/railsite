use std::env;
use std::env::current_dir;
use std::convert::Infallible;
use clap::{App, crate_authors, crate_version};
use hyper::{Body, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use raildata::load::report::Failed;
use railsite::config::Config;
use railsite::http::Request;
use railsite::site::SiteBase;


async fn process(
    request: hyper::Request<Body>,
    base: SiteBase,
) -> Result<Response<Body>, Infallible> {
    Ok(base.process(Request::new(request)))
}

async fn _main() -> Result<(), Failed> {
    let cur_dir = match current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            eprintln!(
                "Fatal: cannot get current directory ({}). Aborting.",
                err
            );
            return Err(Failed)
        }
    };

    let config = Config::from_arg_matches(
        &Config::config_args(
            App::new("railsite")
                .version(crate_version!())
                .author(crate_authors!())
                .about("the railwayhistory.org server")
        ).get_matches(),
        &cur_dir
    )?;

    let base = SiteBase::load(&config)?;

    eprintln!("Listening on {}", config.listen);

    let make_svc = make_service_fn(move |_conn| {
        let base = base.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |r| {
                process(r, base.clone())
            }))
        }
    });

    let server = Server::bind(&config.listen).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
        return Err(Failed)
    }

    Ok(())
}


#[tokio::main]
async fn main() {
    if let Err(_) = _main().await {
        std::process::exit(1)
    }
}

