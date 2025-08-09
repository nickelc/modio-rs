use std::future::IntoFuture;

use serde_derive::Serialize;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::id::{CommentId, GameId, ModId};
use crate::types::mods::Comment;

/// Edit a comment for a mod.
pub struct EditModComment<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    comment_id: CommentId,
    fields: EditModCommentFields<'a>,
}

#[derive(Serialize)]
struct EditModCommentFields<'a> {
    content: &'a str,
}

impl<'a> EditModComment<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        game_id: GameId,
        mod_id: ModId,
        comment_id: CommentId,
        content: &'a str,
    ) -> Self {
        Self {
            http,
            game_id,
            mod_id,
            comment_id,
            fields: EditModCommentFields { content },
        }
    }
}

impl IntoFuture for EditModComment<'_> {
    type Output = Output<Comment>;
    type IntoFuture = ResponseFuture<Comment>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::EditModComment {
            game_id: self.game_id,
            mod_id: self.mod_id,
            comment_id: self.comment_id,
        };

        match RequestBuilder::from_route(&route).form(&self.fields) {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}
