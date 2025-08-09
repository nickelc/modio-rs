use std::future::IntoFuture;

use serde_derive::Serialize;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::id::{GameId, ModId};
use crate::types::Message;

/// Submit a positive or negative rating for a mod.
pub struct SubmitModRating<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    fields: AddModRatingFields,
}

#[derive(Serialize)]
struct AddModRatingFields {
    rating: i8,
}

impl<'a> SubmitModRating<'a> {
    pub(crate) const fn new(http: &'a Client, game_id: GameId, mod_id: ModId) -> Self {
        Self {
            http,
            game_id,
            mod_id,
            fields: AddModRatingFields { rating: 1 },
        }
    }

    pub const fn positive(mut self) -> Self {
        self.fields.rating = 1;
        self
    }

    pub const fn negative(mut self) -> Self {
        self.fields.rating = -1;
        self
    }

    pub const fn reset(mut self) -> Self {
        self.fields.rating = 0;
        self
    }
}

impl IntoFuture for SubmitModRating<'_> {
    type Output = Output<Message>;
    type IntoFuture = ResponseFuture<Message>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::RateMod {
            game_id: self.game_id,
            mod_id: self.mod_id,
        };
        match RequestBuilder::from_route(&route).form(&self.fields) {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}
