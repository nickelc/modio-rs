use std::future::IntoFuture;

use serde_derive::Serialize;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::id::{CommentId, GameId, ModId};
use crate::types::mods::Comment;

/// Update the Karma rating in single increments or decrements for a mod comment.
pub struct UpdateModCommentKarma<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    comment_id: CommentId,
    fields: UpdateModCommentKarmaFields,
}

#[derive(Serialize)]
struct UpdateModCommentKarmaFields {
    karma: i8,
}

impl<'a> UpdateModCommentKarma<'a> {
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
            fields: UpdateModCommentKarmaFields { karma: 1 },
        }
    }

    pub const fn positive(mut self) -> Self {
        self.fields.karma = 1;
        self
    }

    pub const fn negative(mut self) -> Self {
        self.fields.karma = -1;
        self
    }
}

impl IntoFuture for UpdateModCommentKarma<'_> {
    type Output = Output<Comment>;
    type IntoFuture = ResponseFuture<Comment>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::UpdateModCommentKarma {
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
