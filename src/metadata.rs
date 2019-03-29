//! Mod metadata KVP interface
use futures::future;
use url::form_urlencoded;

use crate::error::Error;
use crate::prelude::*;
use crate::types::mods::MetadataMap;

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

    fn path(&self) -> String {
        format!("/games/{}/mods/{}/metadatakvp", self.game, self.mod_id)
    }

    /// Return the metadata key value pairs for a mod that this `Metadata` refers to.
    pub fn get(&self) -> Future<MetadataMap> {
        #[derive(Deserialize)]
        struct KV {
            metakey: String,
            metavalue: String,
        }

        Box::new(
            self.modio
                .stream::<KV>(&self.path())
                .fold(MetadataMap::new(), |mut map, kv| {
                    map.entry(kv.metakey)
                        .or_insert_with(Vec::new)
                        .push(kv.metavalue);
                    future::ok::<_, Error>(map)
                }),
        )
    }

    /// Add metadata for a mod that this `Metadata` refers to.
    pub fn add(&self, metadata: &MetadataMap) -> Future<()> {
        token_required!(self.modio);
        Box::new(
            self.modio
                .post::<ModioMessage, _>(&self.path(), metadata.to_query_string())
                .map(|_| ()),
        )
    }

    /// Delete metadata for a mod that this `Metadata` refers to.
    pub fn delete(&self, metadata: &MetadataMap) -> Future<()> {
        token_required!(self.modio);
        self.modio.delete(&self.path(), metadata.to_query_string())
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
