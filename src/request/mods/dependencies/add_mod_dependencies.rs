use std::future::IntoFuture;

use serde_derive::Serialize;

use crate::client::Client;
use crate::request::{ArrayParams, Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::id::{GameId, ModId};
use crate::types::Message;

/// Add mod dependencies required by a mod.
pub struct AddModDependencies<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    fields: AddModDependenciesFields<'a>,
}

#[derive(Serialize)]
struct AddModDependenciesFields<'a> {
    #[serde(flatten)]
    dependencies: ArrayParams<'a, ModId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sync: Option<bool>,
}

impl<'a> AddModDependenciesFields<'a> {
    const fn new(deps: &'a [ModId]) -> Self {
        Self {
            dependencies: ArrayParams::new("dependencies[]", deps),
            sync: None,
        }
    }
}

impl<'a> AddModDependencies<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        game_id: GameId,
        mod_id: ModId,
        deps: &'a [ModId],
    ) -> Self {
        Self {
            http,
            game_id,
            mod_id,
            fields: AddModDependenciesFields::new(deps),
        }
    }

    /// Replace all existing dependencies with the new ones.
    pub const fn replace(mut self, value: bool) -> Self {
        self.fields.sync = Some(value);
        self
    }
}

impl IntoFuture for AddModDependencies<'_> {
    type Output = Output<Message>;
    type IntoFuture = ResponseFuture<Message>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::AddModDependencies {
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

    use super::{AddModDependenciesFields, ModId};

    #[test]
    pub fn serialize_fields() {
        let deps = [ModId::new(1), ModId::new(2)];
        let mut fields = AddModDependenciesFields::new(&deps);
        fields.sync = Some(true);

        assert_ser_tokens(
            &fields,
            &[
                Token::Map { len: None },
                Token::Str("dependencies[]"),
                Token::U64(1),
                Token::Str("dependencies[]"),
                Token::U64(2),
                Token::Str("sync"),
                Token::Some,
                Token::Bool(true),
                Token::MapEnd,
            ],
        );

        let qs = serde_urlencoded::to_string(&fields).unwrap();
        assert_eq!("dependencies%5B%5D=1&dependencies%5B%5D=2&sync=true", qs);
    }
}
