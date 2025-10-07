use std::fmt;
use std::future::IntoFuture;

use bytes::Bytes;
use futures_util::TryStream;
use http::header::CONTENT_RANGE;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::files::multipart::{UploadId, UploadPart};
use crate::types::id::{GameId, ModId};

pub struct AddMultipartUploadPart<'a, S> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    upload_id: UploadId,
    range: ContentRange,
    stream: S,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ContentRange {
    pub start: u64,
    pub end: u64,
    pub total: u64,
}

impl<'a, S> AddMultipartUploadPart<'a, S> {
    pub(crate) const fn new(
        http: &'a Client,
        game_id: GameId,
        mod_id: ModId,
        upload_id: UploadId,
        range: ContentRange,
        stream: S,
    ) -> Self
    where
        S: TryStream + Send + 'static,
        S::Ok: Into<Bytes>,
        S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        Self {
            http,
            game_id,
            mod_id,
            upload_id,
            range,
            stream,
        }
    }
}

impl fmt::Display for ContentRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bytes {}-{}/{}", self.start, self.end, self.total)
    }
}

impl<S> IntoFuture for AddMultipartUploadPart<'_, S>
where
    S: TryStream + Send + 'static,
    S::Ok: Into<Bytes>,
    S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    type Output = Output<UploadPart>;
    type IntoFuture = ResponseFuture<UploadPart>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::AddMultipartUploadPart {
            game_id: self.game_id,
            mod_id: self.mod_id,
            upload_id: self.upload_id,
        };

        let range = self.range.to_string();
        let builder = RequestBuilder::from_route(&route).header(CONTENT_RANGE, range);
        match builder.stream(self.stream) {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}
