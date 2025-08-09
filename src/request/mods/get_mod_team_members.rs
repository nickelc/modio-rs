use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{Filter, Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::id::{GameId, ModId};
use crate::types::mods::TeamMember;
use crate::types::List;

/// Get all users that are part of a mod team.
pub struct GetModTeamMembers<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    filter: Option<Filter>,
}

impl<'a> GetModTeamMembers<'a> {
    pub(crate) const fn new(http: &'a Client, game_id: GameId, mod_id: ModId) -> Self {
        Self {
            http,
            game_id,
            mod_id,
            filter: None,
        }
    }

    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }
}

impl IntoFuture for GetModTeamMembers<'_> {
    type Output = Output<List<TeamMember>>;
    type IntoFuture = ResponseFuture<List<TeamMember>>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::GetModTeamMembers {
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
