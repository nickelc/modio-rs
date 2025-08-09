use std::collections::BTreeMap;
use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{ArrayParams, Output, RequestBuilder, Route};
use crate::response::{NoContent, ResponseFuture};
use crate::types::id::{GameId, ModId};
use crate::types::mods::MetadataMap;

/// Delete key value pairs metadata defined for a mod.
pub struct DeleteModMetadata<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    fields: DeleteModMetadataFields,
}

struct DeleteModMetadataFields {
    metadata: MetadataMap,
}

impl DeleteModMetadataFields {
    fn flatten(&self) -> Vec<String> {
        let sorted = self.metadata.iter().collect::<BTreeMap<_, _>>();
        let mut metadata = Vec::new();
        for (key, values) in sorted {
            if values.is_empty() {
                metadata.push(key.to_owned());
                continue;
            }
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

impl<'a> DeleteModMetadata<'a> {
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
            fields: DeleteModMetadataFields { metadata },
        }
    }
}

impl IntoFuture for DeleteModMetadata<'_> {
    type Output = Output<NoContent>;
    type IntoFuture = ResponseFuture<NoContent>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::DeleteModMetadata {
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

    use super::{ArrayParams, DeleteModMetadataFields, MetadataMap};

    #[test]
    pub fn serialize_fields() {
        let mut metadata = MetadataMap::new();
        metadata.insert(
            String::from("aaa"),
            vec![String::from("bbb"), String::from("ccc")],
        );
        metadata.insert(String::from("ddd"), vec![]);
        metadata.insert(
            String::from("eee"),
            vec![String::from("fff"), String::from("ggg")],
        );
        let fields = DeleteModMetadataFields { metadata };
        let flatten = fields.flatten();
        let params = ArrayParams::new("metadata[]", &flatten);

        assert_ser_tokens(
            &params,
            &[
                Token::Map { len: Some(5) },
                Token::Str("metadata[]"),
                Token::Str("aaa:bbb"),
                Token::Str("metadata[]"),
                Token::Str("aaa:ccc"),
                Token::Str("metadata[]"),
                Token::Str("ddd"),
                Token::Str("metadata[]"),
                Token::Str("eee:fff"),
                Token::Str("metadata[]"),
                Token::Str("eee:ggg"),
                Token::MapEnd,
            ],
        );

        let qs = serde_urlencoded::to_string(&params).unwrap();
        assert_eq!(
            "metadata%5B%5D=aaa%3Abbb&metadata%5B%5D=aaa%3Accc&metadata%5B%5D=ddd&metadata%5B%5D=eee%3Afff&metadata%5B%5D=eee%3Aggg",
            qs
        );
    }
}
