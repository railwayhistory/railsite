
use std::sync::Arc;
use actix_web::{server, App};
use raildata::store::Store;
use crate::views;


//------------ Railsite ------------------------------------------------------

pub struct Railsite(App<Arc<Store>>);

impl Railsite {
    pub fn new(store: Arc<Store>) -> Self {
        let app = App::with_state(store);
        Railsite(Self::resources(app))
    }

    fn resources(app: App<Arc<Store>>) -> App<Arc<Store>> {
        let res = app.default_resource(|r| r.f(views::errors::not_found))
            .resource("/", |r| r.get().f(views::index::index));
        static_resources(res)
            
    }
}

impl server::IntoHttpHandler for Railsite {
    type Handler = <App<Arc<Store>> as server::IntoHttpHandler>::Handler;

    fn into_handler(self) -> Self::Handler {
        self.0.into_handler()
    }
}


//------------ Definition of Statics -----------------------------------------

static CSS: &[u8] = b"text/css";
static JS: &[u8] = b"text/javascript";

fn static_resources<S: 'static>(app: App<S>) -> App<S> {
    statics!(app,
        "style.css" => CSS,
        "js/bootstrap.min.js" => JS,
        "js/jquery.min.js" => JS,
        "js/popper.min.js" => JS,
    )
}


//------------ HttpRequest ---------------------------------------------------

pub type HttpRequest = actix_web::HttpRequest<Arc<Store>>;

