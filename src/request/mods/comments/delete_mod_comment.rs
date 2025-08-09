use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::{NoContent, ResponseFuture};
use crate::types::id::{CommentId, GameId, ModId};

/// Delete a comment from a mod profile.
pub struct DeleteModComment<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    comment_id: CommentId,
}

impl<'a> DeleteModComment<'a> {
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

impl IntoFuture for DeleteModComment<'_> {
    type Output = Output<NoContent>;
    type IntoFuture = ResponseFuture<NoContent>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::DeleteModComment {
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
