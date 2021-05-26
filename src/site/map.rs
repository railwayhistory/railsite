
use hyper::http::response::Builder as ResponseBuilder;
use crate::html;
use crate::html::target::RenderText;
use crate::http::{GetRequest, Response};
use super::Site;

impl Site {
    pub(super) fn process_map(
        &self, mut request: GetRequest
    ) -> Result<Response, GetRequest> {
        if request.path().segment() != "map" {
            return Err(request)
        }

        if request.path_mut().next().is_none() {
            return Ok(
                html::map::full_size(self).into_ok()
            )
        }

        if request.path().segment() == "tiles" {
            return Ok(self.map_tiles(request))
        }
        
        Ok(self.not_found(request))
    }

    fn map_tiles(&self, mut request: GetRequest) -> Response {
        let map = match request.path_mut().next() {
            Some(map) => map,
            None => return self.not_found(request),
        };

        // We only have a map for now: "current".
        if map != "current" {
            return self.not_found(request)
        }

        if request.path_mut().next().is_none() {
            return self.not_found(request)
        }

        match self.map().tile_for_path(request.path().remaining()) {
            Ok(Some(tile)) => {
                ResponseBuilder::new()
                .header("Content-Type", tile.content_type())
                .body(tile.data().into())
                .unwrap()
            }
            _ => {
                self.not_found(request)
            }
        }
    }

    pub fn link_tile_base(&self) -> impl RenderText + '_ {
        self.link("map/tiles/")
    }
}

