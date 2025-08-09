use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::id::{GameId, ModId};
use crate::types::mods::Dependency;
use crate::types::List;

/// Get all dependencies a mod has selected.
pub struct GetModDependencies<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    recursive: Option<bool>,
}

impl<'a> GetModDependencies<'a> {
    pub(crate) const fn new(http: &'a Client, game_id: GameId, mod_id: ModId) -> Self {
        Self {
            http,
            game_id,
            mod_id,
            recursive: None,
        }
    }
}

impl IntoFuture for GetModDependencies<'_> {
    type Output = Output<List<Dependency>>;
    type IntoFuture = ResponseFuture<List<Dependency>>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::GetModDependencies {
            game_id: self.game_id,
            mod_id: self.mod_id,
            recursive: self.recursive,
        };
        match RequestBuilder::from_route(&route).empty() {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}
