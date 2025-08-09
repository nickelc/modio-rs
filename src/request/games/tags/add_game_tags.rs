use std::future::IntoFuture;

use serde::ser::{Serialize, SerializeMap, Serializer};

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
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

struct AddGameTagsFields<'a> {
    name: &'a str,
    kind: TagType,
    hidden: Option<bool>,
    locked: Option<bool>,
    tags: &'a [&'a str],
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
            fields: AddGameTagsFields {
                name,
                kind,
                hidden: None,
                locked: None,
                tags,
            },
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

impl Serialize for AddGameTagsFields<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let len = 2
            + usize::from(self.hidden.is_some())
            + usize::from(self.locked.is_some())
            + self.tags.len();
        let mut map = serializer.serialize_map(Some(len))?;
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("type", &self.kind)?;
        if let Some(hidden) = self.hidden {
            map.serialize_entry("hidden", &hidden)?;
        }
        if let Some(locked) = self.locked {
            map.serialize_entry("locked", &locked)?;
        }
        for t in self.tags {
            map.serialize_entry("tags[]", t)?;
        }
        map.end()
    }
}
