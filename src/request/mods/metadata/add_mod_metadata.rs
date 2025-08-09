use std::collections::BTreeMap;
use std::future::IntoFuture;

use serde::ser::{Serialize, SerializeMap, Serializer};

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::id::{GameId, ModId};
use crate::types::mods::MetadataMap;
use crate::types::Message;

/// Add metadata for a mod as searchable key value pairs.
pub struct AddModMetadata<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    fields: AddModMetadataFields,
}

struct AddModMetadataFields {
    metadata: MetadataMap,
}

impl AddModMetadataFields {
    fn flatten(&self) -> Vec<String> {
        let sorted = self.metadata.iter().collect::<BTreeMap<_, _>>();
        let mut metadata = Vec::new();
        for (key, values) in sorted {
            for value in values {
                let mut v = key.clone();
                v.push(':');
                v.push_str(value);
                metadata.push(v);
            }
        }
        metadata
    }
}

impl<'a> AddModMetadata<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        game_id: GameId,
        mod_id: ModId,
        metadata: MetadataMap,
    ) -> Self {
        Self {
            http,
            game_id,
            mod_id,
            fields: AddModMetadataFields { metadata },
        }
    }
}

impl IntoFuture for AddModMetadata<'_> {
    type Output = Output<Message>;
    type IntoFuture = ResponseFuture<Message>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::AddModMetadata {
            game_id: self.game_id,
            mod_id: self.mod_id,
        };
        match RequestBuilder::from_route(&route).form(&self.fields) {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}

impl Serialize for AddModMetadataFields {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let values = self.flatten();
        let mut map = serializer.serialize_map(Some(values.len()))?;
        for value in values {
            map.serialize_entry("metadata[]", &value)?;
        }
        map.end()
    }
}
