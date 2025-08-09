use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::id::{CommentId, GameId, ModId};
use crate::types::mods::Comment;

/// Get a comment posted on a mod profile.
pub struct GetModComment<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    comment_id: CommentId,
}

impl<'a> GetModComment<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        game_id: GameId,
        mod_id: ModId,
        comment_id: CommentId,
    ) -> Self {
        Self {
            http,
            game_id,
            mod_id,
            comment_id,
        }
    }
}

impl IntoFuture for GetModComment<'_> {
    type Output = Output<Comment>;
    type IntoFuture = ResponseFuture<Comment>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::GetModComment {
            game_id: self.game_id,
            mod_id: self.mod_id,
            comment_id: self.comment_id,
        };
        match RequestBuilder::from_route(&route).empty() {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}
