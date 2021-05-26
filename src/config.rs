//! Configuration.

use std::fs;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use clap::{App, Arg, ArgMatches};
use raildata::load::report::Failed;
use serde::Deserialize;


//------------ Config --------------------------------------------------------

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    /// Path to the directory containing the database.
    pub database: PathBuf,

    /// Path to the directory containing the documentation.
    pub documentation: PathBuf,

    /// Path to the directory containing the map.
    pub map: PathBuf,

    /// Path to the directory containing the cache.
    pub cache: PathBuf,

    /// The socket address to listen on for the HTTP service.
    pub listen: SocketAddr,

    /// The base URL of the site.
    pub url_base: String,
}

impl Config {
    pub fn config_args<'a: 'b, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
        app
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .takes_value(true)
             .value_name("PATH")
             .help("Read base configuration from this file")
        )
    }

    pub fn from_arg_matches(
        matches: &ArgMatches,
        cur_dir: &Path,
    ) -> Result<Self, Failed> {
        let config_path = cur_dir.join(
            matches.value_of("config").unwrap_or("railsite.toml")
        );

        let config = match fs::read(&config_path) {
            Ok(config) => config,
            Err(err) => {
                eprintln!("Config file {}: {}", config_path.display(), err);
                return Err(Failed)
            }
        };

        let mut config: Self = match toml::de::from_slice(&config) {
            Ok(config) => config,
            Err(err) => {
                eprintln!("Config file {}: {}", config_path.display(), err);
                return Err(Failed)
            }
        };

        // If we could read the file, there surely is a parent.
        let config_dir = config_path.parent().unwrap();

        config.database = config_dir.join(config.database);
        config.documentation = config_dir.join(config.documentation);
        config.map = config_dir.join(config.map);
        config.cache = config_dir.join(config.cache);

        Ok(config)
    }
}

