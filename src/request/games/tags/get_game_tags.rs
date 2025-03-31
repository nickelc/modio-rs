use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::games::TagOption;
use crate::types::id::GameId;
use crate::types::List;
use crate::util::{Paginate, Paginator};

/// Get all tags for the corresponding game, that can be applied to any of its mods.
pub struct GetGameTags<'a> {
    http: &'a Client,
    game_id: GameId,
    show_hidden_tags: Option<bool>,
}

impl<'a> GetGameTags<'a> {
    pub(crate) const fn new(http: &'a Client, game_id: GameId) -> Self {
        Self {
            http,
            game_id,
            show_hidden_tags: None,
        }
    }

    /// Show the hidden tags associated with the given game.
    pub const fn show_hidden_tags(mut self, value: bool) -> Self {
        self.show_hidden_tags = Some(value);
        self
    }
}

impl IntoFuture for GetGameTags<'_> {
    type Output = Output<List<TagOption>>;
    type IntoFuture = ResponseFuture<List<TagOption>>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::GetGameTags {
            game_id: self.game_id,
            show_hidden_tags: self.show_hidden_tags,
        };
        match RequestBuilder::from_route(&route).empty() {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}

impl<'a> Paginate<'a> for GetGameTags<'a> {
    type Output = TagOption;

    fn paged(&'a self) -> Paginator<'a, Self::Output> {
        let route = Route::GetGameTags {
            game_id: self.game_id,
            show_hidden_tags: self.show_hidden_tags,
        };

        Paginator::new(self.http, route, None)
    }
}
