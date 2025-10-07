use std::future::IntoFuture;

use serde_derive::Serialize;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::files::multipart::UploadSession;
use crate::types::id::{GameId, ModId};

/// Create a session for uploading files in multiple part.
pub struct CreateMultipartUploadSession<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    fields: CreateMultipartUploadSessionFields<'a>,
}

#[derive(Serialize)]
struct CreateMultipartUploadSessionFields<'a> {
    filename: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    nonce: Option<&'a str>,
}

impl<'a> CreateMultipartUploadSession<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        game_id: GameId,
        mod_id: ModId,
        filename: &'a str,
    ) -> Self {
        Self {
            http,
            game_id,
            mod_id,
            fields: CreateMultipartUploadSessionFields {
                filename,
                nonce: None,
            },
        }
    }

    /// An optional nonce to provide to prevent duplicate upload sessions
    /// from being created concurrently.
    ///
    /// Maximun 64 characters (Recommended: SHA-256)
    pub const fn nonce(mut self, nonce: &'a str) -> Self {
        self.fields.nonce = Some(nonce);
        self
    }
}

impl IntoFuture for CreateMultipartUploadSession<'_> {
    type Output = Output<UploadSession>;
    type IntoFuture = ResponseFuture<UploadSession>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::CreateMultipartUploadSession {
            game_id: self.game_id,
            mod_id: self.mod_id,
        };
        match RequestBuilder::from_route(&route).form(&self.fields) {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}
