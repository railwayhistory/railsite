use std::env::current_dir;
use clap::{App, crate_authors, crate_version};
use railsite::server;
use railsite::config::Config;
use railsite::state::ServerState;

#[tokio::main]
async fn main() {
    let cur_dir = match current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            eprintln!(
                "Fatal: cannot get current directory ({}). Aborting.",
                err
            );
            return
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
    );
    let config = match config {
        Ok(config) => config,
        Err(_) => return
    };

    let state = match ServerState::load(&config) {
        Ok(state) => state.into_arc(),
        Err(_) => return
    };

    eprintln!("Listening on {}", config.listen);

    server::serve(&config, state).await
}

