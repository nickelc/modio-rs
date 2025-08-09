use std::future::IntoFuture;

use serde::ser::{Serialize, SerializeMap, Serializer};

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::{NoContent, ResponseFuture};
use crate::types::id::{GameId, ModId};

/// Delete mod dependencies a mod has selected.
pub struct DeleteModDependencies<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    fields: DeleteModDependenciesFields<'a>,
}

struct DeleteModDependenciesFields<'a> {
    dependencies: &'a [ModId],
}

impl<'a> DeleteModDependencies<'a> {
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
            fields: DeleteModDependenciesFields { dependencies: deps },
        }
    }
}

impl IntoFuture for DeleteModDependencies<'_> {
    type Output = Output<NoContent>;
    type IntoFuture = ResponseFuture<NoContent>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::DeleteModDependencies {
            game_id: self.game_id,
            mod_id: self.mod_id,
        };
        match RequestBuilder::from_route(&route).form(&self.fields) {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}

impl Serialize for DeleteModDependenciesFields<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.dependencies.len()))?;
        for dep in self.dependencies {
            map.serialize_entry("dependencies[]", dep)?;
        }
        map.end()
    }
}
