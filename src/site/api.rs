use std::cmp;
use std::str::FromStr;
use json::object;
use raildata::library::LinkTarget;
use crate::html::target::RenderText;
use crate::http::{Response, Request};
use crate::i18n::Lang;
use super::Site;

impl Site {
    pub fn process_api(
        &self, mut request: Request
    ) -> Result<Response, Request> {
        if request.path().segment() != "api" {
            return Err(request)
        }
        if request.path_mut().next().is_none() {
            return Ok(self.not_found(request))
        }

        if request.path().segment() == "search" {
            Ok(self.process_api_search(request))
        }
        else {
            Ok(self.not_found(request))
        }
    }

    fn process_api_search(&self, request: Request) -> Response {
        if !request.is_get() {
            return self.method_not_allowed(request)
        }

        let q = match request.query().get("q") {
            Some(q) if !q.is_empty() => q,
            _ => {
                return request.ok_json(object!{ items: [] })
            }
        };

        let lang = Lang::from_code(
            request.query().get("lang").map(AsRef::as_ref).unwrap_or("en")
        );

        let count = request.query().get("num").and_then(|num| {
            usize::from_str(num).ok()
        }).map(|count| cmp::min(count, 100)).unwrap_or(20);

        let res = self.catalogue().names.search(q, count)
        .map(|(_, link)| {
            let doc = self.library().resolve(link);
            object! {
                type: format!("{}", doc.doctype()),
                title: doc.name(lang.into()),
                key: doc.key().as_str(),
                url: self.link_document(doc).format(),
            }
        }).collect::<Vec<_>>();

        request.ok_json(object! {
            items: res
        })
    }
}

