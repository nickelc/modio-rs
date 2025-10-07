use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::files::multipart::{UploadId, UploadPart};
use crate::types::id::{GameId, ModId};
use crate::types::List;
use crate::util::{Paginate, Paginator};

pub struct GetMultipartUploadParts<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    upload_id: UploadId,
}

impl<'a> GetMultipartUploadParts<'a> {
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

impl IntoFuture for GetMultipartUploadParts<'_> {
    type Output = Output<List<UploadPart>>;
    type IntoFuture = ResponseFuture<List<UploadPart>>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::GetMultipartUploadParts {
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

impl<'a> Paginate<'a> for GetMultipartUploadParts<'a> {
    type Output = UploadPart;

    fn paged(&'a self) -> Paginator<'a, Self::Output> {
        let route = Route::GetMultipartUploadParts {
            game_id: self.game_id,
            mod_id: self.mod_id,
            upload_id: self.upload_id,
        };
        Paginator::new(self.http, route, None)
    }
}
