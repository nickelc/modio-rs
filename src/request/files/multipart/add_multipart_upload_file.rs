use std::future::IntoFuture;

use serde_derive::Serialize;

use crate::client::Client;
use crate::error;
use crate::request::multipart::{Form, Part};
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::files::multipart::UploadId;
use crate::types::files::File;
use crate::types::id::{GameId, ModId};

pub struct AddMultipartUploadFile<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    fields: AddMultipartUploadFileFields<'a>,
}

#[derive(Serialize)]
struct AddMultipartUploadFileFields<'a> {
    upload_id: UploadId,
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

impl<'a> AddMultipartUploadFile<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        game_id: GameId,
        mod_id: ModId,
        upload_id: UploadId,
    ) -> Self {
        Self {
            http,
            game_id,
            mod_id,
            fields: AddMultipartUploadFileFields {
                upload_id,
                active: None,
                changelog: None,
                filehash: None,
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

impl IntoFuture for AddMultipartUploadFile<'_> {
    type Output = Output<File>;
    type IntoFuture = ResponseFuture<File>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::AddFile {
            game_id: self.game_id,
            mod_id: self.mod_id,
        };

        let mut form = Form::new();
        form = form.text("upload_id", self.fields.upload_id.to_string());

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
