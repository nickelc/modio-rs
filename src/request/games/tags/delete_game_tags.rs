use std::future::IntoFuture;

use serde_derive::Serialize;

use crate::client::Client;
use crate::request::{ArrayParams, Output, RequestBuilder, Route};
use crate::response::{NoContent, ResponseFuture};
use crate::types::id::GameId;

/// Delete an entire group of tags or individual tags.
pub struct DeleteGameTags<'a> {
    http: &'a Client,
    game_id: GameId,
    fields: DeleteGameTagsFields<'a>,
}

#[derive(Serialize)]
struct DeleteGameTagsFields<'a> {
    name: &'a str,
    #[serde(flatten)]
    tags: ArrayParams<'a, &'a str>,
}

impl<'a> DeleteGameTagsFields<'a> {
    const ALL_TAGS: ArrayParams<'static, &'static str> = ArrayParams::new("tags[]", &[""]);

    const fn new(name: &'a str) -> Self {
        Self {
            name,
            tags: Self::ALL_TAGS,
        }
    }

    const fn set_tags(&mut self, tags: &'a [&'a str]) {
        if tags.is_empty() {
            self.tags = Self::ALL_TAGS;
        } else {
            self.tags = ArrayParams::new("tags[]", tags);
        }
    }
}

impl<'a> DeleteGameTags<'a> {
    pub(crate) const fn new(http: &'a Client, game_id: GameId, name: &'a str) -> Self {
        Self {
            http,
            game_id,
            fields: DeleteGameTagsFields::new(name),
        }
    }

    pub const fn tags(mut self, tags: &'a [&'a str]) -> Self {
        self.fields.set_tags(tags);
        self
    }
}

impl IntoFuture for DeleteGameTags<'_> {
    type Output = Output<NoContent>;
    type IntoFuture = ResponseFuture<NoContent>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::DeleteGameTags {
            game_id: self.game_id,
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

    use super::DeleteGameTagsFields;

    #[test]
    pub fn serialize_fields() {
        let mut fields = DeleteGameTagsFields::new("aaa");
        fields.set_tags(&["bbb", "ccc"]);

        assert_ser_tokens(
            &fields,
            &[
                Token::Map { len: None },
                Token::Str("name"),
                Token::Str("aaa"),
                Token::Str("tags[]"),
                Token::Str("bbb"),
                Token::Str("tags[]"),
                Token::Str("ccc"),
                Token::MapEnd,
            ],
        );

        let qs = serde_urlencoded::to_string(&fields).unwrap();
        assert_eq!("name=aaa&tags%5B%5D=bbb&tags%5B%5D=ccc", qs);
    }

    #[test]
    pub fn serialize_fields_all_tags() {
        let fields = DeleteGameTagsFields::new("aaa");

        assert_ser_tokens(
            &fields,
            &[
                Token::Map { len: None },
                Token::Str("name"),
                Token::Str("aaa"),
                Token::Str("tags[]"),
                Token::Str(""),
                Token::MapEnd,
            ],
        );

        let qs = serde_urlencoded::to_string(&fields).unwrap();
        assert_eq!("name=aaa&tags%5B%5D=", qs);
    }
}
