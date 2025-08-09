use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::id::{GameId, ModId};
use crate::types::mods::Statistics;

/// Get mod stats for a mod.
pub struct GetModStats<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
}

impl<'a> GetModStats<'a> {
    pub(crate) const fn new(http: &'a Client, game_id: GameId, mod_id: ModId) -> Self {
        Self {
            http,
            game_id,
            mod_id,
        }
    }
}

impl IntoFuture for GetModStats<'_> {
    type Output = Output<Statistics>;
    type IntoFuture = ResponseFuture<Statistics>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::GetModStats {
            game_id: self.game_id,
            mod_id: self.mod_id,
        };
        match RequestBuilder::from_route(&route).empty() {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}
