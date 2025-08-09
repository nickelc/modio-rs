use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{Filter, Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::games::Game;
use crate::types::List;

/// Get all games.
pub struct GetGames<'a> {
    http: &'a Client,
    show_hidden_tags: Option<bool>,
    filter: Option<Filter>,
}

impl<'a> GetGames<'a> {
    pub(crate) const fn new(http: &'a Client) -> Self {
        Self {
            http,
            show_hidden_tags: None,
            filter: None,
        }
    }

    /// Show the hidden tags associated with the given game.
    pub const fn show_hidden_tags(mut self, value: bool) -> Self {
        self.show_hidden_tags = Some(value);
        self
    }

    /// Set the filter for the request.
    ///
    /// See [Filters and sorting](super::filters).
    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }
}

impl IntoFuture for GetGames<'_> {
    type Output = Output<List<Game>>;
    type IntoFuture = ResponseFuture<List<Game>>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::GetGames {
            show_hidden_tags: self.show_hidden_tags,
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
