use std::net::ToSocketAddrs;
use hyper::error::Result;
use hyper::server::{Listening, Request, Response, Server};


fn handle(_req: Request, res: Response) {
    let _ = res.send(b"Hello world!");
}

pub struct Site(Server);

impl Site {
    pub fn new<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        Ok(Site(Server::http(addr)?))
    }

    pub fn serve(self) -> Result<Listening> {
        self.0.handle(handle)
    }
}
