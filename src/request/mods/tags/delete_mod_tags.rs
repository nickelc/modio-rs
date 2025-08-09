use std::future::IntoFuture;

use serde_derive::Serialize;

use crate::client::Client;
use crate::request::{ArrayParams, Output, RequestBuilder, Route};
use crate::response::{NoContent, ResponseFuture};
use crate::types::id::{GameId, ModId};

/// Delete tags from a mod profile.
pub struct DeleteModTags<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    fields: DeleteModTagsFields<'a>,
}

#[derive(Serialize)]
struct DeleteModTagsFields<'a> {
    #[serde(flatten)]
    tags: ArrayParams<'a, &'a str>,
}

impl<'a> DeleteModTagsFields<'a> {
    const fn new(tags: &'a [&'a str]) -> Self {
        Self {
            tags: ArrayParams::new("tags[]", tags),
        }
    }
}

impl<'a> DeleteModTags<'a> {
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
            fields: DeleteModTagsFields::new(tags),
        }
    }
}

impl IntoFuture for DeleteModTags<'_> {
    type Output = Output<NoContent>;
    type IntoFuture = ResponseFuture<NoContent>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::DeleteModTags {
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

    use super::DeleteModTagsFields;

    #[test]
    pub fn serialize_fields() {
        let fields = DeleteModTagsFields::new(&["aaa", "bbb"]);

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
