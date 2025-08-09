use std::future::IntoFuture;

use serde::ser::{Serialize, SerializeMap, Serializer};

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::id::{GameId, ModId};
use crate::types::Message;

/// Add tags to a mod profile.
pub struct AddModTags<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    fields: AddModTagsFields<'a>,
}

struct AddModTagsFields<'a> {
    tags: &'a [&'a str],
}

impl<'a> AddModTags<'a> {
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
            fields: AddModTagsFields { tags },
        }
    }
}

impl IntoFuture for AddModTags<'_> {
    type Output = Output<Message>;
    type IntoFuture = ResponseFuture<Message>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::AddModTags {
            game_id: self.game_id,
            mod_id: self.mod_id,
        };
        match RequestBuilder::from_route(&route).form(&self.fields) {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}

impl Serialize for AddModTagsFields<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.tags.len()))?;
        for t in self.tags {
            map.serialize_entry("tags[]", t)?;
        }
        map.end()
    }
}
