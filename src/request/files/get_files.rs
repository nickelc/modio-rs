use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{Filter, Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::files::File;
use crate::types::id::{GameId, ModId};
use crate::types::List;

/// Get all files that are published for a mod.
pub struct GetFiles<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    filter: Option<Filter>,
}

impl<'a> GetFiles<'a> {
    pub(crate) const fn new(http: &'a Client, game_id: GameId, mod_id: ModId) -> Self {
        Self {
            http,
            game_id,
            mod_id,
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

impl IntoFuture for GetFiles<'_> {
    type Output = Output<List<File>>;
    type IntoFuture = ResponseFuture<List<File>>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::GetFiles {
            game_id: self.game_id,
            mod_id: self.mod_id,
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
