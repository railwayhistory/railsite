
use std::io::Write;
use hyper::Result;
use ::http::{Context, HtmlResponse};

pub fn index<E>(context: Context<E>) -> Result<HtmlResponse> {
    let mut res = HtmlResponse::new_forbidden();
    write!(res, "<html><head><title>Forbidden</title></head>\
                 <body><h1>Forbidden</h1><p>{}</p></body></html>",
           context.request().path())?;
    Ok(res)                   
}

