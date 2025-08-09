use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::{NoContent, ResponseFuture};
use crate::types::id::{GameId, ModId};

/// Delete a mod profile.
pub struct DeleteMod<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
}

impl<'a> DeleteMod<'a> {
    pub(crate) const fn new(http: &'a Client, game_id: GameId, mod_id: ModId) -> Self {
        Self {
            http,
            game_id,
            mod_id,
        }
    }
}

impl IntoFuture for DeleteMod<'_> {
    type Output = Output<NoContent>;
    type IntoFuture = ResponseFuture<NoContent>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::DeleteMod {
            game_id: self.game_id,
            mod_id: self.mod_id,
        };

        match RequestBuilder::from_route(&route).empty() {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}
