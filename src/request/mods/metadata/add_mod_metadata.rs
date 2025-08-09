use std::collections::BTreeMap;
use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{ArrayParams, Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::id::{GameId, ModId};
use crate::types::mods::MetadataMap;
use crate::types::Message;

/// Add metadata for a mod as searchable key value pairs.
pub struct AddModMetadata<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    fields: AddModMetadataFields,
}

struct AddModMetadataFields {
    metadata: MetadataMap,
}

impl AddModMetadataFields {
    fn flatten(&self) -> Vec<String> {
        let sorted = self.metadata.iter().collect::<BTreeMap<_, _>>();
        let mut metadata = Vec::new();
        for (key, values) in sorted {
            for value in values {
                let mut v = key.clone();
                v.push(':');
                v.push_str(value);
                metadata.push(v);
            }
        }
        metadata
    }
}

impl<'a> AddModMetadata<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        game_id: GameId,
        mod_id: ModId,
        metadata: MetadataMap,
    ) -> Self {
        Self {
            http,
            game_id,
            mod_id,
            fields: AddModMetadataFields { metadata },
        }
    }
}

impl IntoFuture for AddModMetadata<'_> {
    type Output = Output<Message>;
    type IntoFuture = ResponseFuture<Message>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::AddModMetadata {
            game_id: self.game_id,
            mod_id: self.mod_id,
        };
        let metadata = self.fields.flatten();
        let form = ArrayParams::new("metadata[]", &metadata);
        match RequestBuilder::from_route(&route).form(&form) {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_test::{assert_ser_tokens, Token};

    use super::{AddModMetadataFields, ArrayParams, MetadataMap};

    #[test]
    pub fn serialize_fields() {
        let mut metadata = MetadataMap::new();
        metadata.insert(
            String::from("aaa"),
            vec![String::from("bbb"), String::from("ccc")],
        );
        metadata.insert(
            String::from("ddd"),
            vec![String::from("eee"), String::from("fff")],
        );
        let fields = AddModMetadataFields { metadata };
        let flatten = fields.flatten();
        let params = ArrayParams::new("metadata[]", &flatten);

        assert_ser_tokens(
            &params,
            &[
                Token::Map { len: Some(4) },
                Token::Str("metadata[]"),
                Token::Str("aaa:bbb"),
                Token::Str("metadata[]"),
                Token::Str("aaa:ccc"),
                Token::Str("metadata[]"),
                Token::Str("ddd:eee"),
                Token::Str("metadata[]"),
                Token::Str("ddd:fff"),
                Token::MapEnd,
            ],
        );

        let qs = serde_urlencoded::to_string(&params).unwrap();
        assert_eq!(
            "metadata%5B%5D=aaa%3Abbb&metadata%5B%5D=aaa%3Accc&metadata%5B%5D=ddd%3Aeee&metadata%5B%5D=ddd%3Afff",
            qs
        );
    }
}
