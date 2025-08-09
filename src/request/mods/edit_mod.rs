use std::future::IntoFuture;

use serde_derive::Serialize;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::id::{GameId, ModId};
use crate::types::mods::{CommunityOptions, CreditOptions, MaturityOption, Mod};

/// Edit details for a mod.
pub struct EditMod<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    fields: EditModFields<'a>,
}

#[derive(Serialize)]
struct EditModFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<&'a str>,
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

impl<'a> EditMod<'a> {
    pub(crate) const fn new(http: &'a Client, game_id: GameId, mod_id: ModId) -> Self {
        Self {
            http,
            game_id,
            mod_id,
            fields: EditModFields {
                status: None,
                visibility: None,
                name: None,
                name_id: None,
                summary: None,
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

    pub const fn name(mut self, name: &'a str) -> Self {
        self.fields.name = Some(name);
        self
    }

    pub const fn name_id(mut self, name_id: &'a str) -> Self {
        self.fields.name_id = Some(name_id);
        self
    }

    pub const fn summary(mut self, summary: &'a str) -> Self {
        self.fields.summary = Some(summary);
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

    pub const fn status(mut self, status: u8) -> Self {
        self.fields.status = Some(status);
        self
    }

    pub const fn visible(mut self, visible: bool) -> Self {
        self.fields.visibility = Some(if visible { 1 } else { 0 });
        self
    }
}

impl IntoFuture for EditMod<'_> {
    type Output = Output<Mod>;
    type IntoFuture = ResponseFuture<Mod>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::EditMod {
            game_id: self.game_id,
            mod_id: self.mod_id,
        };
        match RequestBuilder::from_route(&route).form(&self.fields) {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}
