use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::{NoContent, ResponseFuture};
use crate::types::files::multipart::UploadId;
use crate::types::id::{GameId, ModId};

pub struct DeleteMultipartUploadSession<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    upload_id: UploadId,
}

impl<'a> DeleteMultipartUploadSession<'a> {
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

impl IntoFuture for DeleteMultipartUploadSession<'_> {
    type Output = Output<NoContent>;
    type IntoFuture = ResponseFuture<NoContent>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::DeleteMultipartUploadSession {
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
