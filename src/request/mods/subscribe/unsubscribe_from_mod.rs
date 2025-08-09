use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::{NoContent, ResponseFuture};
use crate::types::id::{GameId, ModId};

/// Unsubscribe the authenticated user from a mod.
pub struct UnsubscribeFromMod<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
}

impl<'a> UnsubscribeFromMod<'a> {
    pub(crate) const fn new(http: &'a Client, game_id: GameId, mod_id: ModId) -> Self {
        Self {
            http,
            game_id,
            mod_id,
        }
    }
}

impl IntoFuture for UnsubscribeFromMod<'_> {
    type Output = Output<NoContent>;
    type IntoFuture = ResponseFuture<NoContent>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::UnsubscribeFromMod {
            game_id: self.game_id,
            mod_id: self.mod_id,
        };
        match RequestBuilder::from_route(&route).empty() {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}
