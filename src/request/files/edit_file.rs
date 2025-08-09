use std::future::IntoFuture;

use serde_derive::Serialize;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::files::File;
use crate::types::id::{FileId, GameId, ModId};

/// Edit the details of a modfile.
pub struct EditFile<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    file_id: FileId,
    fields: EditFileFields<'a>,
}

#[derive(Serialize)]
struct EditFileFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    changelog: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_blob: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<&'a str>,
}

impl<'a> EditFile<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        game_id: GameId,
        mod_id: ModId,
        file_id: FileId,
    ) -> Self {
        Self {
            http,
            game_id,
            mod_id,
            file_id,
            fields: EditFileFields {
                active: None,
                changelog: None,
                metadata_blob: None,
                version: None,
            },
        }
    }

    pub const fn active(mut self, active: bool) -> Self {
        self.fields.active = Some(active);
        self
    }

    pub const fn changelog(mut self, changelog: &'a str) -> Self {
        self.fields.changelog = Some(changelog);
        self
    }

    pub const fn metadata_blob(mut self, metadata: &'a str) -> Self {
        self.fields.metadata_blob = Some(metadata);
        self
    }

    pub const fn version(mut self, version: &'a str) -> Self {
        self.fields.version = Some(version);
        self
    }
}

impl IntoFuture for EditFile<'_> {
    type Output = Output<File>;
    type IntoFuture = ResponseFuture<File>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::EditFile {
            game_id: self.game_id,
            mod_id: self.mod_id,
            file_id: self.file_id,
        };
        match RequestBuilder::from_route(&route).form(&self.fields) {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}
