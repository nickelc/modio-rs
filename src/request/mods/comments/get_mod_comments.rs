use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{Filter, Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::id::{GameId, ModId};
use crate::types::mods::Comment;
use crate::types::List;
use crate::util::{Paginate, Paginator};

/// Get all comments posted on a mod profile.
pub struct GetModComments<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    filter: Option<Filter>,
}

impl<'a> GetModComments<'a> {
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

impl IntoFuture for GetModComments<'_> {
    type Output = Output<List<Comment>>;
    type IntoFuture = ResponseFuture<List<Comment>>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::GetModComments {
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

impl<'a> Paginate<'a> for GetModComments<'a> {
    type Output = Comment;

    fn paged(&'a self) -> Paginator<'a, Self::Output> {
        let route = Route::GetModComments {
            game_id: self.game_id,
            mod_id: self.mod_id,
        };
        Paginator::new(self.http, route, self.filter.clone())
    }
}
