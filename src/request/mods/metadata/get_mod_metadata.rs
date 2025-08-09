use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::id::{GameId, ModId};
use crate::types::mods::MetadataMap;

/// Get all metadata for a mod as searchable key value pairs.
pub struct GetModMetadata<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
}

impl<'a> GetModMetadata<'a> {
    pub(crate) const fn new(http: &'a Client, game_id: GameId, mod_id: ModId) -> Self {
        Self {
            http,
            game_id,
            mod_id,
        }
    }
}

impl IntoFuture for GetModMetadata<'_> {
    type Output = Output<MetadataMap>;
    type IntoFuture = ResponseFuture<MetadataMap>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::GetModMetadata {
            game_id: self.game_id,
            mod_id: self.mod_id,
        };
        match RequestBuilder::from_route(&route).empty() {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}
