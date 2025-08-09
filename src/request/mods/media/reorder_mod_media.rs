use std::future::IntoFuture;

use serde::ser::{Serialize, SerializeMap, Serializer};

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::{NoContent, ResponseFuture};
use crate::types::id::{GameId, ModId};

/// Reorder images, sketchfab or youtube links from a mod profile.
pub struct ReorderModMedia<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    fields: ReorderModMediaFields<'a>,
}

struct ReorderModMediaFields<'a> {
    images: Option<&'a [&'a str]>,
    youtube: Option<&'a [&'a str]>,
    sketchfab: Option<&'a [&'a str]>,
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
        self.fields.images = Some(images);
        self
    }

    pub const fn youtube(mut self, youtube: &'a [&'a str]) -> Self {
        self.fields.youtube = Some(youtube);
        self
    }

    pub const fn sketchfab(mut self, sketchfab: &'a [&'a str]) -> Self {
        self.fields.sketchfab = Some(sketchfab);
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

impl Serialize for ReorderModMediaFields<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let len = self.images.as_ref().map(|s| s.len()).unwrap_or_default()
            + self.youtube.as_ref().map(|s| s.len()).unwrap_or_default()
            + self.sketchfab.as_ref().map(|s| s.len()).unwrap_or_default();

        let mut map = serializer.serialize_map(Some(len))?;

        if let Some(images) = self.images {
            for e in images {
                map.serialize_entry("images[]", e)?;
            }
        }
        if let Some(youtube) = self.youtube {
            for e in youtube {
                map.serialize_entry("youtube[]", e)?;
            }
        }
        if let Some(sketchfab) = self.sketchfab {
            for e in sketchfab {
                map.serialize_entry("sketchfab[]", e)?;
            }
        }

        map.end()
    }
}
