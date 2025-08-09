use std::future::IntoFuture;

use serde_derive::Serialize;

use crate::client::Client;
use crate::request::{ArrayParams, Output, RequestBuilder, Route};
use crate::response::{NoContent, ResponseFuture};
use crate::types::id::{GameId, ModId};

/// Reorder images, sketchfab or youtube links from a mod profile.
pub struct ReorderModMedia<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    fields: ReorderModMediaFields<'a>,
}

#[derive(Serialize)]
struct ReorderModMediaFields<'a> {
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    images: Option<ArrayParams<'a, &'a str>>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    youtube: Option<ArrayParams<'a, &'a str>>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    sketchfab: Option<ArrayParams<'a, &'a str>>,
}

impl<'a> ReorderModMediaFields<'a> {
    const fn set_images(&mut self, images: &'a [&'a str]) {
        self.images = Some(ArrayParams::new("images[]", images));
    }

    const fn set_youtube(&mut self, youtube: &'a [&'a str]) {
        self.youtube = Some(ArrayParams::new("youtube[]", youtube));
    }

    const fn set_sketchfab(&mut self, sketchfab: &'a [&'a str]) {
        self.sketchfab = Some(ArrayParams::new("sketchfab[]", sketchfab));
    }
}

impl<'a> ReorderModMedia<'a> {
    pub(crate) const fn new(http: &'a Client, game_id: GameId, mod_id: ModId) -> Self {
        Self {
            http,
            game_id,
            mod_id,
            fields: ReorderModMediaFields {
                images: None,
                youtube: None,
                sketchfab: None,
            },
        }
    }

    pub const fn images(mut self, images: &'a [&'a str]) -> Self {
        self.fields.set_images(images);
        self
    }

    pub const fn youtube(mut self, youtube: &'a [&'a str]) -> Self {
        self.fields.set_youtube(youtube);
        self
    }

    pub const fn sketchfab(mut self, sketchfab: &'a [&'a str]) -> Self {
        self.fields.set_sketchfab(sketchfab);
        self
    }
}

impl IntoFuture for ReorderModMedia<'_> {
    type Output = Output<NoContent>;
    type IntoFuture = ResponseFuture<NoContent>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::ReorderModMedia {
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

    use super::ReorderModMediaFields;

    #[test]
    pub fn serialize_fields() {
        let mut fields = ReorderModMediaFields {
            images: None,
            youtube: None,
            sketchfab: None,
        };
        fields.set_images(&["aaa", "bbb"]);
        fields.set_sketchfab(&["ccc", "ddd"]);

        assert_ser_tokens(
            &fields,
            &[
                Token::Map { len: None },
                Token::Str("images[]"),
                Token::Str("aaa"),
                Token::Str("images[]"),
                Token::Str("bbb"),
                Token::Str("sketchfab[]"),
                Token::Str("ccc"),
                Token::Str("sketchfab[]"),
                Token::Str("ddd"),
                Token::MapEnd,
            ],
        );

        let qs = serde_urlencoded::to_string(&fields).unwrap();
        assert_eq!(
            "images%5B%5D=aaa&images%5B%5D=bbb&sketchfab%5B%5D=ccc&sketchfab%5B%5D=ddd",
            qs
        );
    }
}
