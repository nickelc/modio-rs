use std::future::IntoFuture;

use serde_derive::Serialize;

use crate::client::Client;
use crate::request::{ArrayParams, Output, RequestBuilder, Route};
use crate::response::{NoContent, ResponseFuture};
use crate::types::id::{GameId, ModId};

/// Delete mod dependencies a mod has selected.
pub struct DeleteModDependencies<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    fields: DeleteModDependenciesFields<'a>,
}

#[derive(Serialize)]
struct DeleteModDependenciesFields<'a> {
    #[serde(flatten)]
    dependencies: ArrayParams<'a, ModId>,
}

impl<'a> DeleteModDependenciesFields<'a> {
    const fn new(deps: &'a [ModId]) -> Self {
        Self {
            dependencies: ArrayParams::new("dependencies[]", deps),
        }
    }
}

impl<'a> DeleteModDependencies<'a> {
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
            fields: DeleteModDependenciesFields::new(deps),
        }
    }
}

impl IntoFuture for DeleteModDependencies<'_> {
    type Output = Output<NoContent>;
    type IntoFuture = ResponseFuture<NoContent>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::DeleteModDependencies {
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

    use super::{DeleteModDependenciesFields, ModId};

    #[test]
    pub fn serialize_fields() {
        let deps = [ModId::new(1), ModId::new(2)];
        let fields = DeleteModDependenciesFields::new(&deps);

        assert_ser_tokens(
            &fields,
            &[
                Token::Map { len: None },
                Token::Str("dependencies[]"),
                Token::U64(1),
                Token::Str("dependencies[]"),
                Token::U64(2),
                Token::MapEnd,
            ],
        );

        let qs = serde_urlencoded::to_string(&fields).unwrap();
        assert_eq!("dependencies%5B%5D=1&dependencies%5B%5D=2", qs);
    }
}
