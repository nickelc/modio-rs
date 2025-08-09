use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::files::File;
use crate::types::id::{FileId, GameId, ModId};

/// Get a file.
pub struct GetFile<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    file_id: FileId,
}

impl<'a> GetFile<'a> {
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

impl IntoFuture for GetFile<'_> {
    type Output = Output<File>;
    type IntoFuture = ResponseFuture<File>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::GetFile {
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
