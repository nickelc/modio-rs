use std::future::IntoFuture;

use serde_derive::Serialize;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::id::{CommentId, GameId, ModId};
use crate::types::mods::Comment;

/// Add a comment for a mod.
pub struct AddModComment<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    fields: AddModCommentFields<'a>,
}

#[derive(Serialize)]
struct AddModCommentFields<'a> {
    content: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_id: Option<CommentId>,
}

impl<'a> AddModComment<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        game_id: GameId,
        mod_id: ModId,
        content: &'a str,
    ) -> Self {
        Self {
            http,
            game_id,
            mod_id,
            fields: AddModCommentFields {
                content,
                reply_id: None,
            },
        }
    }

    pub const fn reply_id(mut self, reply_id: CommentId) -> Self {
        self.fields.reply_id = Some(reply_id);
        self
    }
}

impl IntoFuture for AddModComment<'_> {
    type Output = Output<Comment>;
    type IntoFuture = ResponseFuture<Comment>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::AddModComment {
            game_id: self.game_id,
            mod_id: self.mod_id,
        };
        match RequestBuilder::from_route(&route).form(&self.fields) {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}
