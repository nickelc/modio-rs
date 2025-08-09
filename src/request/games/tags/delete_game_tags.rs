use std::future::IntoFuture;

use serde::ser::{Serialize, SerializeMap, Serializer};

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::{NoContent, ResponseFuture};
use crate::types::id::GameId;

/// Delete an entire group of tags or individual tags.
pub struct DeleteGameTags<'a> {
    http: &'a Client,
    game_id: GameId,
    fields: DeleteGameTagsFields<'a>,
}

struct DeleteGameTagsFields<'a> {
    name: &'a str,
    tags: Option<&'a [&'a str]>,
}

impl<'a> DeleteGameTags<'a> {
    pub(crate) const fn new(http: &'a Client, game_id: GameId, name: &'a str) -> Self {
        Self {
            http,
            game_id,
            fields: DeleteGameTagsFields { name, tags: None },
        }
    }

    pub const fn tags(mut self, tags: &'a [&'a str]) -> Self {
        self.fields.tags = Some(tags);
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

impl Serialize for DeleteGameTagsFields<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let len = self.tags.as_ref().map_or(1, |t| t.len());
        let mut map = serializer.serialize_map(Some(len + 1))?;
        map.serialize_entry("name", &self.name)?;
        if let Some(tags) = self.tags {
            for t in tags {
                map.serialize_entry("tags[]", t)?;
            }
        } else {
            map.serialize_entry("tags[]", "")?;
        }
        map.end()
    }
}
