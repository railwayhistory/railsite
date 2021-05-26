use std::path::Path;
use std::sync::Arc;
use raildata::library::Library;
use raildata::load::load_tree;
use raildata::load::report::Failed;
use crate::catalogue::Catalogue;
use crate::config::Config;
use crate::documentation::Documentation;
use crate::html::target::{RenderText, Text};
use crate::http::{GetRequest, Request, Response};
use crate::map::Map;
use crate::i18n::Lang;


//----------- SiteBase -------------------------------------------------------

#[derive(Clone)]
pub struct SiteBase {
    library: Library,
    catalogue: Catalogue,
    documentation: Documentation,
    map: Map,
    base: Arc<String>,
}

impl SiteBase {
    pub fn load(config: &Config) -> Result<Self, Failed> {
        let library = Self::load_library(&config.database)?;
        let catalogue = Catalogue::new(&library);
        Ok(SiteBase {
            library, catalogue,
            documentation: Documentation::load(config)?,
            map: Map::load(config)?,
            base: Arc::new(config.url_base.clone())
        })
    }

    fn load_library(path: &Path) -> Result<Library, Failed> {
        let library = match load_tree(path) {
            Ok(library) => library,
            Err(err) => {
                eprintln!("Database {}: {} errors.", path.display(), err.len());
                for item in err.iter() {
                    println!("{}", item)
                }
                return Err(Failed)
            }
        };
        eprintln!("Finished loading database.");
        Ok(library)
    }

    pub fn process(&self, request: Request) -> Response {
        Site {
            base: self.clone(),
            lang: request.lang()
        }.process(request)
    }
}


//----------- Site -----------------------------------------------------------

pub struct Site {
    base: SiteBase,
    lang: Lang,
}

impl Site {
    pub fn library(&self) -> &Library {
        &self.base.library
    }

    pub fn catalogue(&self) -> &Catalogue {
        &self.base.catalogue
    }

    pub fn documentation(&self) -> &Documentation {
        &self.base.documentation
    }

    pub fn map(&self) -> &Map {
        &self.base.map
    }

    pub fn lang(&self) -> Lang {
        self.lang
    }

    pub fn link<'s, P: RenderText + 's>(&'s self, path: P) -> SiteLink<'s, P> {
        SiteLink { site: self, path }
    }
}

impl Site {
    pub fn process(&self, request: Request) -> Response {
        self.process_api(request)
        .or_else(|request| self.process_not_get(request))
        .or_else(|request| self.process_statics(request))
        .or_else(|request| self.process_index(request))
        .or_else(|request| self.process_doc(request))
        .or_else(|request| self.process_document(request))
        .or_else(|request| self.process_map(request))
        .unwrap_or_else(|request| self.not_found(request))
    }

    fn process_not_get(
        &self, request: Request
    ) -> Result<Response, GetRequest> {
        match request.get() {
            Ok(get) => Err(get),
            Err(request) => Ok(self.method_not_allowed(request))
        }
    }
}


//------------ SiteLink ------------------------------------------------------

pub struct SiteLink<'a, T> {
    site: &'a Site,
    path: T
}

impl<'a, T: RenderText> RenderText for SiteLink<'a, T> {
    fn render(self, target: &mut Text) {
        self.site.base.base.as_str().render(target);
        self.path.render(target);
    }
}

