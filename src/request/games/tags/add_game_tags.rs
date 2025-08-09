use std::future::IntoFuture;

use serde_derive::Serialize;

use crate::client::Client;
use crate::request::{ArrayParams, Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::games::TagType;
use crate::types::id::GameId;
use crate::types::Message;

/// Add tags which can by applied to mod profiles.
pub struct AddGameTags<'a> {
    http: &'a Client,
    game_id: GameId,
    fields: AddGameTagsFields<'a>,
}

#[derive(Serialize)]
struct AddGameTagsFields<'a> {
    name: &'a str,
    #[serde(rename = "type")]
    kind: TagType,
    #[serde(skip_serializing_if = "Option::is_none")]
    hidden: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locked: Option<bool>,
    #[serde(flatten)]
    tags: ArrayParams<'a, &'a str>,
}

impl<'a> AddGameTagsFields<'a> {
    const fn new(name: &'a str, kind: TagType, tags: &'a [&'a str]) -> Self {
        Self {
            name,
            kind,
            hidden: None,
            locked: None,
            tags: ArrayParams::new("tags[]", tags),
        }
    }
}

impl<'a> AddGameTags<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        game_id: GameId,
        name: &'a str,
        kind: TagType,
        tags: &'a [&'a str],
    ) -> Self {
        Self {
            http,
            game_id,
            fields: AddGameTagsFields::new(name, kind, tags),
        }
    }

    pub const fn hidden(mut self, hidden: bool) -> Self {
        self.fields.hidden = Some(hidden);
        self
    }

    pub const fn locked(mut self, locked: bool) -> Self {
        self.fields.locked = Some(locked);
        self
    }
}

impl IntoFuture for AddGameTags<'_> {
    type Output = Output<Message>;
    type IntoFuture = ResponseFuture<Message>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::AddGameTags {
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

    use super::{AddGameTagsFields, TagType};

    #[test]
    pub fn serialize_fields() {
        let fields = AddGameTagsFields::new("aaa", TagType::Checkboxes, &["bbb", "ccc"]);

        assert_ser_tokens(
            &fields,
            &[
                Token::Map { len: None },
                Token::Str("name"),
                Token::Str("aaa"),
                Token::Str("type"),
                Token::UnitVariant {
                    name: "TagType",
                    variant: "checkboxes",
                },
                Token::Str("tags[]"),
                Token::Str("bbb"),
                Token::Str("tags[]"),
                Token::Str("ccc"),
                Token::MapEnd,
            ],
        );

        let qs = serde_urlencoded::to_string(&fields).unwrap();
        assert_eq!("name=aaa&type=checkboxes&tags%5B%5D=bbb&tags%5B%5D=ccc", qs);
    }
}
