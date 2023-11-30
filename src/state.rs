use std::sync::Arc;
use headers::{Cookie, HeaderMapExt};
use httools::request::{Request, RequestQuery};
use httools::response::{Response, ResponseBuilder};
use raildata::catalogue::Catalogue;
use raildata::load::load_tree;
use raildata::load::report::{Failed, Stage};
use raildata::store::FullStore;
use crate::config::Config;
use crate::lang::Lang;

//------------ ServerState ---------------------------------------------------

pub struct ServerState {
    store: FullStore,
    catalogue: Catalogue,
    url_base: String,
}

impl ServerState {
    pub fn load(config: &Config) -> Result<Self, Failed> {
        let store = match load_tree(&config.database) {
            Ok(store) => store,
            Err(mut err) => {
                err.sort();

                if err.has_stage(Stage::Parse) {
                    println!("{} errors.", err.stage_count(Stage::Parse));
                    for item in err.iter() {
                        if item.stage() == Stage::Parse {
                            eprintln!("{}", item)
                        }
                    }
                }
                else {
                    eprintln!("{} errors.", err.len());
                    for item in err.iter() {
                        eprintln!("{}", item)
                    }
                }
                return Err(Failed)
            }
        };
        let store = match store.into_full_store() {
            Ok(store) => store,
            Err(mut err) => {
                err.sort();
                eprintln!("{} errors.", err.len());
                for item in err.iter() {
                    eprintln!("{}", item)
                }
                return Err(Failed)
            }
        };
        let catalogue = match Catalogue::generate(&store) {
            Ok(catalogue) => catalogue,
            Err(mut err) => {
                err.sort();
                eprintln!("{} errors.", err.len());
                for item in err.iter() {
                    eprintln!("{}", item)
                }
                return Err(Failed)
            }
        };

        Ok(Self { store, catalogue, url_base: config.url_base.clone() })
    }

    pub fn into_arc(self) -> Arc<Self> {
        Arc::new(self)
    }

    pub fn store(&self) -> &FullStore {
        &self.store
    }

    pub fn catalogue(&self) -> &Catalogue {
        &self.catalogue
    }

    pub fn url_base(&self) -> &str {
        &self.url_base
    }
}


//------------ RequestState --------------------------------------------------

pub struct RequestState {
    server: Arc<ServerState>,
    query: RequestQuery,
    lang: Lang,
}

impl RequestState {
    pub fn from_request(
        request: &Request, server: Arc<ServerState>
    ) -> Result<Self, Response> {
        let query = request.query();
        let lang = Self::determine_lang(request, &query)?;
        Ok(RequestState {
            server, query, lang
        })
    }

    /// Determine the language.
    ///
    /// Returns the language and whether it was changed.
    fn determine_lang(
        request: &Request,
        query: &RequestQuery,
    ) -> Result<Lang, Response> {
        // If we have a "lang" attribute in the query, we use that -- this is
        // how we switch languages.
        if let Some(lang) = query.get_first("lang") {
            return Ok(Lang::from_code(lang))
        }

        // If we have a "lang" cookie, we use that.
        if let Some(cookies) = request.headers().typed_get::<Cookie>() {
            if let Some(lang) = cookies.get("lang") {
                return Ok(Lang::from_code(lang))
            }
        }

        // Otherwise we will do the default for now.
        Ok(Lang::default())
    }

    pub fn store(&self) -> &FullStore {
        self.server.store()
    }

    pub fn catalogue(&self) -> &Catalogue {
        self.server.catalogue()
    }

    pub fn url_base(&self) -> &str {
        self.server.url_base()
    }

    pub fn query(&self) -> &RequestQuery {
        &self.query
    }

    pub fn lang(&self) -> Lang {
        self.lang
    }

    pub fn response(&self) -> ResponseBuilder {
        ResponseBuilder::new().set_static_cookie(self.lang.cookie())
    }
}

