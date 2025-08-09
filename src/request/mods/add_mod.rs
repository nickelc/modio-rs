use std::future::IntoFuture;
use std::path::Path;

use serde_derive::Serialize;

use crate::client::Client;
use crate::error;
use crate::request::multipart::{Form, Part};
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::id::GameId;
use crate::types::mods::{CommunityOptions, CreditOptions, MaturityOption, Mod};

/// Add a mod.
pub struct AddMod<'a> {
    http: &'a Client,
    game_id: GameId,
    logo: Part,
    fields: AddModFields<'a>,
}

#[derive(Serialize)]
struct AddModFields<'a> {
    name: &'a str,
    summary: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    homepage_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stock: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maturity_option: Option<MaturityOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    community_options: Option<CommunityOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credit_options: Option<CreditOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_blob: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<&'a [&'a str]>,
}

impl<'a> AddMod<'a> {
    pub(crate) fn new(
        http: &'a Client,
        game_id: GameId,
        name: &'a str,
        summary: &'a str,
        logo: impl AsRef<Path>,
    ) -> Self {
        let logo = Part::file(logo, "logo.png").mime(mime::IMAGE_STAR);

        Self {
            http,
            game_id,
            logo,
            fields: AddModFields {
                name,
                summary,
                visibility: None,
                name_id: None,
                description: None,
                homepage_url: None,
                stock: None,
                maturity_option: None,
                community_options: None,
                credit_options: None,
                metadata_blob: None,
                tags: None,
            },
        }
    }

    pub const fn name_id(mut self, name_id: &'a str) -> Self {
        self.fields.name_id = Some(name_id);
        self
    }

    pub const fn description(mut self, description: &'a str) -> Self {
        self.fields.description = Some(description);
        self
    }

    pub const fn homepage_url(mut self, homepage_url: &'a str) -> Self {
        self.fields.homepage_url = Some(homepage_url);
        self
    }

    pub const fn stock(mut self, stock: u32) -> Self {
        self.fields.stock = Some(stock);
        self
    }

    pub const fn maturity_option(mut self, option: MaturityOption) -> Self {
        self.fields.maturity_option = Some(option);
        self
    }

    pub const fn community_options(mut self, options: CommunityOptions) -> Self {
        self.fields.community_options = Some(options);
        self
    }

    pub const fn credit_options(mut self, options: CreditOptions) -> Self {
        self.fields.credit_options = Some(options);
        self
    }

    pub const fn metadata_blob(mut self, metadata: &'a str) -> Self {
        self.fields.metadata_blob = Some(metadata);
        self
    }

    pub const fn tags(mut self, tags: &'a [&'a str]) -> Self {
        self.fields.tags = Some(tags);
        self
    }

    pub const fn visible(mut self, visible: bool) -> Self {
        self.fields.visibility = Some(if visible { 1 } else { 0 });
        self
    }
}

impl IntoFuture for AddMod<'_> {
    type Output = Output<Mod>;
    type IntoFuture = ResponseFuture<Mod>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::AddMod {
            game_id: self.game_id,
        };

        let mut form = Form::new();

        form = form.text("name", self.fields.name.to_owned());
        form = form.text("summary", self.fields.summary.to_owned());
        form = form.part("logo", self.logo);

        if let Some(value) = self.fields.name_id {
            form = form.text("name_id", value.to_owned());
        }
        if let Some(value) = self.fields.description {
            form = form.text("description", value.to_owned());
        }
        if let Some(value) = self.fields.homepage_url {
            form = form.text("homepage_url", value.to_owned());
        }
        if let Some(value) = self.fields.visibility {
            form = form.text("visible", value.to_string());
        }
        if let Some(value) = self.fields.stock {
            form = form.text("stock", value.to_string());
        }
        if let Some(value) = self.fields.maturity_option {
            form = form.text("maturity_option", value.to_string());
        }
        if let Some(value) = self.fields.community_options {
            form = form.text("community_options", value.to_string());
        }
        if let Some(value) = self.fields.credit_options {
            form = form.text("credit_options", value.to_string());
        }
        if let Some(value) = self.fields.metadata_blob {
            form = form.text("metadata_blob", value.to_owned());
        }
        if let Some(tags) = self.fields.tags {
            for tag in tags {
                form = form.text("tags[]", (*tag).to_owned());
            }
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
