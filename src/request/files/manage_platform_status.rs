use std::future::IntoFuture;

use serde_derive::Serialize;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::files::File;
use crate::types::id::{FileId, GameId, ModId};
use crate::types::TargetPlatform;

/// Manage the platform status of a particular modfile.
pub struct ManagePlatformStatus<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    file_id: FileId,
    fields: ManagePlatformStatusFields<'a>,
}

#[derive(Serialize)]
struct ManagePlatformStatusFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    approved: Option<&'a [TargetPlatform]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    denied: Option<&'a [TargetPlatform]>,
}

impl<'a> ManagePlatformStatus<'a> {
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
            fields: ManagePlatformStatusFields {
                approved: None,
                denied: None,
            },
        }
    }
}

impl IntoFuture for ManagePlatformStatus<'_> {
    type Output = Output<File>;
    type IntoFuture = ResponseFuture<File>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::ManagePlatformStatus {
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
