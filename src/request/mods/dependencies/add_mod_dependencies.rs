use std::future::IntoFuture;

use serde::ser::{Serialize, SerializeMap, Serializer};

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::id::{GameId, ModId};
use crate::types::Message;

/// Add mod dependencies required by a mod.
pub struct AddModDependencies<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    fields: AddModDependenciesFields<'a>,
}

struct AddModDependenciesFields<'a> {
    dependencies: &'a [ModId],
    sync: Option<bool>,
}

impl<'a> AddModDependencies<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        game_id: GameId,
        mod_id: ModId,
        deps: &'a [ModId],
    ) -> Self {
        Self {
            http,
            game_id,
            mod_id,
            fields: AddModDependenciesFields {
                dependencies: deps,
                sync: None,
            },
        }
    }

    /// Replace all existing dependencies with the new ones.
    pub const fn replace(mut self, value: bool) -> Self {
        self.fields.sync = Some(value);
        self
    }
}

impl IntoFuture for AddModDependencies<'_> {
    type Output = Output<Message>;
    type IntoFuture = ResponseFuture<Message>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::AddModDependencies {
            game_id: self.game_id,
            mod_id: self.mod_id,
        };
        match RequestBuilder::from_route(&route).form(&self.fields) {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}

impl Serialize for AddModDependenciesFields<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.dependencies.len()))?;
        for dep in self.dependencies {
            map.serialize_entry("dependencies[]", dep)?;
        }
        if let Some(sync) = self.sync {
            map.serialize_entry("sync", &sync)?;
        }
        map.end()
    }
}
