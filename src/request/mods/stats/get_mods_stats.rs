use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{Filter, Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::id::GameId;
use crate::types::mods::Statistics;
use crate::types::List;

/// Get all mod stats for mods of a game.
pub struct GetModsStats<'a> {
    http: &'a Client,
    game_id: GameId,
    filter: Option<Filter>,
}

impl<'a> GetModsStats<'a> {
    pub(crate) const fn new(http: &'a Client, game_id: GameId) -> Self {
        Self {
            http,
            game_id,
            filter: None,
        }
    }

    /// Set the filter for the request.
    ///
    /// See [Filters and sorting](super::filters).
    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }
}

impl IntoFuture for GetModsStats<'_> {
    type Output = Output<List<Statistics>>;
    type IntoFuture = ResponseFuture<List<Statistics>>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::GetModsStats {
            game_id: self.game_id,
        };
        let mut builder = RequestBuilder::from_route(&route);
        if let Some(filter) = self.filter {
            builder = builder.filter(filter);
        }
        match builder.empty() {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}
