use std::sync::Arc;
use futures::future::{FutureResult, IntoFuture};
use hyper;
use hyper::{Body, Request, Response};
use hyper::service::{NewService, Service};
use raildata::store::Store;
use super::errors::serve_404;
use super::index;
use super::statics::serve_statics;

#[derive(Clone)]
pub struct Railsite<M> {
    mount: M,
    store: Arc<Store>,
}

impl<M> Railsite<M> {
    pub fn new(mount: M, store: Store) -> Self {
        Railsite {
            mount,
            store: Arc::new(store)
        }
    }

    pub fn store(&self) -> &Store {
        &self.store
    }
}

impl Service for Railsite {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = hyper::Error;
    type Future = FutureResult<Response<Self::ResBody>, Self::Error>;

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        if let Some(response) = serve_statics(&request) {
            return Ok(response).into_future()
        }
        if request.uri().path() == "/" {
            return Ok(index::index(self, request)).into_future()
        }
        Ok(serve_404(request)).into_future()
    }
}

impl NewService for Railsite {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = hyper::Error;
    type Service = Self;
    type Future = FutureResult<Self::Service, Self::InitError>;
    type InitError = hyper::Error;

    fn new_service(&self) -> Self::Future {
        Ok(self.clone()).into_future()
    }
}

