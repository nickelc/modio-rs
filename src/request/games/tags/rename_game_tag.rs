use std::future::IntoFuture;

use serde_derive::Serialize;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::id::GameId;
use crate::types::Message;

/// Rename an existing tag, updating all mods in the progress.
pub struct RenameGameTag<'a> {
    http: &'a Client,
    game_id: GameId,
    fields: RenameGameTagFields<'a>,
}

#[derive(Serialize)]
struct RenameGameTagFields<'a> {
    from: &'a str,
    to: &'a str,
}

impl<'a> RenameGameTag<'a> {
    pub(crate) const fn new(http: &'a Client, game_id: GameId, from: &'a str, to: &'a str) -> Self {
        Self {
            http,
            game_id,
            fields: RenameGameTagFields { from, to },
        }
    }
}

impl IntoFuture for RenameGameTag<'_> {
    type Output = Output<Message>;
    type IntoFuture = ResponseFuture<Message>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::RenameGameTags {
            game_id: self.game_id,
        };
        match RequestBuilder::from_route(&route).form(&self.fields) {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}
