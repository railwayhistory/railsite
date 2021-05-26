//! The map.

use std::sync::Arc;
use raildata::load::report::Failed;
use railmap::features::FeatureSet;
use railmap::tile::{Tile, TileId, TileIdError};
use sled::IVec;
use crate::config::Config;


//------------ Map -----------------------------------------------------------

#[derive(Clone)]
pub struct Map {
    features: Arc<FeatureSet>,
    cache: sled::Db,
}

impl Map {
    pub fn load(config: &Config) -> Result<Self, Failed> {
        let features = railmap::import::load(&config.map).map_err(|err| {
            eprintln!("{}", err);
            Failed
        })?;
        
        let cache = match sled::Config::default().temporary(true).open() {
            Ok(cache) => cache,
            Err(err) => {
                eprintln!("Failed to open cache: {}", err);
                return Err(Failed)
            }
        };

        Ok(Map {
            features: Arc::new(features),
            cache
        })
    }

    pub fn tile_for_path(
        &self, path: &str
    ) -> Result<Option<RenderedTile>, TileIdError> {
        let tile = TileId::from_path(path)?;

        if !tile.is_covered(&self.features) {
            return Ok(None)
        }

        if tile.zoom < 14 {
            Ok(Some(self.cached_tile(tile)))
        }
        else {
            Ok(Some(self.uncached_tile(tile)))
        }
    }

    fn cached_tile(&self, id: TileId) -> RenderedTile {
        let key = format!("{}", id);
        if let Ok(Some(data)) = self.cache.get(&key) {
            return RenderedTile { id, data }
        }

        let res = self.uncached_tile(id);
        let _ = self.cache.insert(&key, res.data.clone());
        res
    }

    fn uncached_tile(&self, id: TileId) -> RenderedTile {
        RenderedTile {
            id,
            data: Tile::new(id).render(&self.features).into()
        }
    }
}


//------------ RenderedTile --------------------------------------------------

#[derive(Clone, Debug)]
pub struct RenderedTile {
    id: TileId,
    data: IVec,
}

impl RenderedTile {
    pub fn content_type(&self) -> &'static str {
        self.id.content_type()
    }

    pub fn data(&self) -> Vec<u8> {
        self.data.as_ref().into()
    }
}

