use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::files::multipart::{UploadId, UploadSession};
use crate::types::id::{GameId, ModId};

pub struct CompleteMultipartUploadSession<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    upload_id: UploadId,
}

impl<'a> CompleteMultipartUploadSession<'a> {
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
            upload_id,
        }
    }
}

impl IntoFuture for CompleteMultipartUploadSession<'_> {
    type Output = Output<UploadSession>;
    type IntoFuture = ResponseFuture<UploadSession>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::CompleteMultipartUploadSession {
            game_id: self.game_id,
            mod_id: self.mod_id,
            upload_id: self.upload_id,
        };
        match RequestBuilder::from_route(&route).empty() {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}
