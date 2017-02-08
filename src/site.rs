use std::net::SocketAddr;
use hyper::Result;
use hyper::server::Http;
use ::http;
use ::views::index;

pub struct Site {
    addr: SocketAddr,
}

impl Site {
    pub fn new(addr: SocketAddr) -> Self {
        Site {
            addr: addr
        }
    }

    pub fn run(&self) -> Result<()> {
        let server = Http::new().bind(&self.addr,
                                      || Ok(http::Site::new((), index)))?;
        server.run()
    }
}
