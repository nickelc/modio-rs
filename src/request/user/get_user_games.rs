use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{Filter, Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::games::Game;
use crate::types::List;

/// Get all games the authenticated user added or is a team member of.
pub struct GetUserGames<'a> {
    http: &'a Client,
    filter: Option<Filter>,
}

impl<'a> GetUserGames<'a> {
    pub(crate) const fn new(http: &'a Client) -> Self {
        Self { http, filter: None }
    }

    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }
}

impl IntoFuture for GetUserGames<'_> {
    type Output = Output<List<Game>>;
    type IntoFuture = ResponseFuture<List<Game>>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::UserGames;
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
