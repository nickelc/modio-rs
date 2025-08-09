use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::id::{GameId, ModId};
use crate::types::mods::Mod;

/// Subscribe the authenticated user to a mod.
pub struct SubscribeToMod<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
}

impl<'a> SubscribeToMod<'a> {
    pub(crate) const fn new(http: &'a Client, game_id: GameId, mod_id: ModId) -> Self {
        Self {
            http,
            game_id,
            mod_id,
        }
    }
}

impl IntoFuture for SubscribeToMod<'_> {
    type Output = Output<Mod>;
    type IntoFuture = ResponseFuture<Mod>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::SubscribeToMod {
            game_id: self.game_id,
            mod_id: self.mod_id,
        };
        match RequestBuilder::from_route(&route).empty() {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}
