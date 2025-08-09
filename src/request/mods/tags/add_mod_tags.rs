use std::future::IntoFuture;

use serde_derive::Serialize;

use crate::client::Client;
use crate::request::{ArrayParams, Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::id::{GameId, ModId};
use crate::types::Message;

/// Add tags to a mod profile.
pub struct AddModTags<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    fields: AddModTagsFields<'a>,
}

#[derive(Serialize)]
struct AddModTagsFields<'a> {
    #[serde(flatten)]
    tags: ArrayParams<'a, &'a str>,
}

impl<'a> AddModTagsFields<'a> {
    const fn new(tags: &'a [&'a str]) -> Self {
        Self {
            tags: ArrayParams::new("tags[]", tags),
        }
    }
}

impl<'a> AddModTags<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        game_id: GameId,
        mod_id: ModId,
        tags: &'a [&'a str],
    ) -> Self {
        Self {
            http,
            game_id,
            mod_id,
            fields: AddModTagsFields::new(tags),
        }
    }
}

impl IntoFuture for AddModTags<'_> {
    type Output = Output<Message>;
    type IntoFuture = ResponseFuture<Message>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::AddModTags {
            game_id: self.game_id,
            mod_id: self.mod_id,
        };
        match RequestBuilder::from_route(&route).form(&self.fields) {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_test::{assert_ser_tokens, Token};

    use super::AddModTagsFields;

    #[test]
    pub fn serialize_fields() {
        let fields = AddModTagsFields::new(&["aaa", "bbb"]);

        assert_ser_tokens(
            &fields,
            &[
                Token::Map { len: None },
                Token::Str("tags[]"),
                Token::Str("aaa"),
                Token::Str("tags[]"),
                Token::Str("bbb"),
                Token::MapEnd,
            ],
        );

        let qs = serde_urlencoded::to_string(&fields).unwrap();
        assert_eq!("tags%5B%5D=aaa&tags%5B%5D=bbb", qs);
    }
}
