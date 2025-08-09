use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::games::Statistics;
use crate::types::id::GameId;

/// Get game stats for a game.
pub struct GetGameStats<'a> {
    http: &'a Client,
    game_id: GameId,
}

impl<'a> GetGameStats<'a> {
    pub(crate) const fn new(http: &'a Client, game_id: GameId) -> Self {
        Self { http, game_id }
    }
}

impl IntoFuture for GetGameStats<'_> {
    type Output = Output<Statistics>;
    type IntoFuture = ResponseFuture<Statistics>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::GetGameStats {
            game_id: self.game_id,
        };
        match RequestBuilder::from_route(&route).empty() {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}
