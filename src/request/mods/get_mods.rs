use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{Filter, Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::id::GameId;
use crate::types::mods::Mod;
use crate::types::List;
use crate::util::{Paginate, Paginator};

/// Get all mods for a game.
pub struct GetMods<'a> {
    http: &'a Client,
    game_id: GameId,
    filter: Option<Filter>,
}

impl<'a> GetMods<'a> {
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

impl IntoFuture for GetMods<'_> {
    type Output = Output<List<Mod>>;
    type IntoFuture = ResponseFuture<List<Mod>>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::GetMods {
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

impl<'a> Paginate<'a> for GetMods<'a> {
    type Output = Mod;

    fn paged(&'a self) -> Paginator<'a, Self::Output> {
        let route = Route::GetMods {
            game_id: self.game_id,
        };
        Paginator::new(self.http, route, self.filter.clone())
    }
}
