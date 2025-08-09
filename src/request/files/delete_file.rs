use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::{NoContent, ResponseFuture};
use crate::types::id::{FileId, GameId, ModId};

/// Delete a modfile.
pub struct DeleteFile<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    file_id: FileId,
}

impl<'a> DeleteFile<'a> {
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
        }
    }
}

impl IntoFuture for DeleteFile<'_> {
    type Output = Output<NoContent>;
    type IntoFuture = ResponseFuture<NoContent>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::DeleteFile {
            game_id: self.game_id,
            mod_id: self.mod_id,
            file_id: self.file_id,
        };

        match RequestBuilder::from_route(&route).empty() {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}
