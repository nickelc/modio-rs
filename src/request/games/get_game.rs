use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::games::Game;
use crate::types::id::GameId;

/// Get a game.
pub struct GetGame<'a> {
    http: &'a Client,
    id: GameId,
    show_hidden_tags: Option<bool>,
}

impl<'a> GetGame<'a> {
    pub(crate) const fn new(http: &'a Client, id: GameId) -> Self {
        Self {
            http,
            id,
            show_hidden_tags: None,
        }
    }

    /// Show the hidden tags associated with the given game.
    pub const fn show_hidden_tags(mut self, value: bool) -> Self {
        self.show_hidden_tags = Some(value);
        self
    }
}

impl IntoFuture for GetGame<'_> {
    type Output = Output<Game>;
    type IntoFuture = ResponseFuture<Game>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::GetGame {
            id: self.id,
            show_hidden_tags: self.show_hidden_tags,
        };
        match RequestBuilder::from_route(&route).empty() {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}
