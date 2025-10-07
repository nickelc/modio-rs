use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::files::multipart::UploadSession;
use crate::types::id::{GameId, ModId};
use crate::types::List;
use crate::util::{Paginate, Paginator};

pub struct GetMultipartUploadSessions<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
}

impl<'a> GetMultipartUploadSessions<'a> {
    pub(crate) const fn new(http: &'a Client, game_id: GameId, mod_id: ModId) -> Self {
        Self {
            http,
            game_id,
            mod_id,
        }
    }
}

impl IntoFuture for GetMultipartUploadSessions<'_> {
    type Output = Output<List<UploadSession>>;
    type IntoFuture = ResponseFuture<List<UploadSession>>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::GetMultipartUploadSessions {
            game_id: self.game_id,
            mod_id: self.mod_id,
        };
        match RequestBuilder::from_route(&route).empty() {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}

impl<'a> Paginate<'a> for GetMultipartUploadSessions<'a> {
    type Output = UploadSession;

    fn paged(&'a self) -> Paginator<'a, Self::Output> {
        let route = Route::GetMultipartUploadSessions {
            game_id: self.game_id,
            mod_id: self.mod_id,
        };
        Paginator::new(self.http, route, None)
    }
}
