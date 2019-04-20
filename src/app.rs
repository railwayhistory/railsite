
use actix_web::{server, App};
use raildata::library::Library;
use crate::views;


//------------ Railsite ------------------------------------------------------

pub struct Railsite(App<Library>);

impl Railsite {
    pub fn new(library: Library) -> Self {
        let app = App::with_state(library);
        Railsite(Self::resources(app))
    }

    fn resources(app: App<Library>) -> App<Library> {
        let res = app.default_resource(|r| r.f(views::errors::not_found))
            .resource("/", |r| r.get().f(views::index::home))
            .resource("/{key}/", |r| r.get().f(views::documents::document))
            .resource(
                "/index/lines/{country}/",
                |r| r.get().f(views::index::lines)
            )
            .resource(
                "/index/lines/",
                |r| r.get().f(views::index::lines)
            );
        static_resources(res)
            
    }
}

impl server::IntoHttpHandler for Railsite {
    type Handler = <App<Library> as server::IntoHttpHandler>::Handler;

    fn into_handler(self) -> Self::Handler {
        self.0.into_handler()
    }
}


//------------ Definition of Statics -----------------------------------------

static CSS: &[u8] = b"text/css";
static JS: &[u8] = b"text/javascript";
static EOT: &[u8] = b"application/vnd.ms-fontobject";
static SVG: &[u8] = b"image/svg+xml";
static TTF: &[u8] = b"application/x-font-truetype";
static WOFF: &[u8] = b"application/font-woff";
static WOFF2: &[u8] = b"application/font-woff2";

fn static_resources<S: 'static>(app: App<S>) -> App<S> {
    statics!(app,
        "style.css" => CSS,
        "js/bootstrap.min.js" => JS,
        "js/jquery.min.js" => JS,
        "js/popper.min.js" => JS,
        "fonts/fa-brands-400.eot" => EOT,
        "fonts/fa-brands-400.svg" => SVG,
        "fonts/fa-brands-400.ttf" => TTF,
        "fonts/fa-brands-400.woff" => WOFF,
        "fonts/fa-brands-400.woff2" => WOFF2,
        "fonts/fa-regular-400.eot" => EOT,
        "fonts/fa-regular-400.svg" => SVG,
        "fonts/fa-regular-400.ttf" => TTF,
        "fonts/fa-regular-400.woff" => WOFF,
        "fonts/fa-regular-400.woff2" => WOFF2,
        "fonts/fa-solid-900.eot" => EOT,
        "fonts/fa-solid-900.svg" => SVG,
        "fonts/fa-solid-900.ttf" => TTF,
        "fonts/fa-solid-900.woff" => WOFF,
        "fonts/fa-solid-900.woff2" => WOFF2,
    )
}


//------------ HttpRequest ---------------------------------------------------

pub type HttpRequest = actix_web::HttpRequest<Library>;

