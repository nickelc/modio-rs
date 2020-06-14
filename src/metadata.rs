//! Mod metadata KVP interface
use futures_util::TryStreamExt;
use serde::Deserialize;
use url::form_urlencoded;

use crate::prelude::*;
pub use crate::types::mods::MetadataMap;

pub struct Metadata {
    modio: Modio,
    game: u32,
    mod_id: u32,
}

impl Metadata {
    pub(crate) fn new(modio: Modio, game: u32, mod_id: u32) -> Self {
        Self {
            modio,
            game,
            mod_id,
        }
    }

    /// Return the metadata key value pairs for a mod that this `Metadata` refers to.
    pub async fn get(self) -> Result<MetadataMap> {
        #[derive(Deserialize)]
        struct KV {
            metakey: String,
            metavalue: String,
        }

        let route = Route::GetModMetadata {
            game_id: self.game,
            mod_id: self.mod_id,
        };
        Query::new(self.modio, route, Default::default())
            .iter()
            .await?
            .try_fold(MetadataMap::new(), |mut map, kv: KV| async {
                map.entry(kv.metakey)
                    .or_insert_with(Vec::new)
                    .push(kv.metavalue);
                Ok(map)
            })
            .await
    }

    /// Add metadata for a mod that this `Metadata` refers to.
    #[allow(clippy::should_implement_trait)]
    pub async fn add(self, metadata: MetadataMap) -> Result<()> {
        let route = Route::AddModMetadata {
            game_id: self.game,
            mod_id: self.mod_id,
        };
        self.modio
            .request(route)
            .body(metadata.to_query_string())
            .send::<Message>()
            .await?;
        Ok(())
    }

    /// Delete metadata for a mod that this `Metadata` refers to.
    pub async fn delete(self, metadata: MetadataMap) -> Result<Deletion> {
        let route = Route::DeleteModMetadata {
            game_id: self.game,
            mod_id: self.mod_id,
        };
        self.modio
            .request(route)
            .body(metadata.to_query_string())
            .send()
            .await
    }
}

impl QueryString for MetadataMap {
    fn to_query_string(&self) -> String {
        let mut ser = form_urlencoded::Serializer::new(String::new());
        for (k, vals) in self.iter() {
            if vals.is_empty() {
                ser.append_pair("metadata[]", k);
            }
            for v in vals {
                ser.append_pair("metadata[]", &format!("{}:{}", k, v));
            }
        }
        ser.finish()
    }
}
