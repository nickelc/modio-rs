use std::future::IntoFuture;
use std::path::Path;

use serde_derive::Serialize;

use crate::client::Client;
use crate::error;
use crate::request::multipart::{Form, Part};
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::files::File;
use crate::types::id::{GameId, ModId};

/// Upload a file for a mod.
pub struct AddFile<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    filedata: Option<Part>,
    fields: AddFileFields<'a>,
}

#[derive(Serialize)]
struct AddFileFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    changelog: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filehash: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_blob: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<&'a str>,
}

impl<'a> AddFile<'a> {
    pub(crate) const fn new(http: &'a Client, game_id: GameId, mod_id: ModId) -> Self {
        Self {
            http,
            game_id,
            mod_id,
            filedata: None,
            fields: AddFileFields {
                active: None,
                changelog: None,
                filehash: None,
                metadata_blob: None,
                version: None,
            },
        }
    }

    pub fn file<P: AsRef<Path>>(self, file: P) -> Self {
        self.file_with_name(file, "modfile.zip")
    }

    pub fn file_with_name<P: AsRef<Path>>(mut self, file: P, filename: &str) -> Self {
        let part = Part::file(file, filename).mime(mime::APPLICATION_OCTET_STREAM);
        self.filedata = Some(part);
        self
    }

    pub const fn active(mut self, active: bool) -> Self {
        self.fields.active = Some(active);
        self
    }

    pub const fn changelog(mut self, changelog: &'a str) -> Self {
        self.fields.changelog = Some(changelog);
        self
    }

    pub const fn filehash(mut self, filehash: &'a str) -> Self {
        self.fields.filehash = Some(filehash);
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

impl IntoFuture for AddFile<'_> {
    type Output = Output<File>;
    type IntoFuture = ResponseFuture<File>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::AddFile {
            game_id: self.game_id,
            mod_id: self.mod_id,
        };

        let mut form = Form::new();
        if let Some(value) = self.filedata {
            form = form.part("filedata", value);
        }
        if let Some(value) = self.fields.active {
            form = form.text("active", value.to_string());
        }
        if let Some(value) = self.fields.changelog {
            form = form.text("changelog", value.to_owned());
        }
        if let Some(value) = self.fields.metadata_blob {
            form = form.text("metadata_blob", value.to_owned());
        }
        if let Some(value) = self.fields.version {
            form = form.text("version", value.to_owned());
        }

        form = match serde_json::to_vec(&self.fields) {
            Ok(json) => form.part("input_json", Part::bytes(json.into())),
            Err(err) => return ResponseFuture::failed(error::builder(err)),
        };

        match RequestBuilder::from_route(&route).multipart(form) {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}
