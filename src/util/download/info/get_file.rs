use std::future::{Future, IntoFuture};
use std::pin::Pin;
use std::task::{ready, Context, Poll};

use futures_util::future::Either;
use http::StatusCode;

use crate::client::Client;
use crate::request::files::GetFile as GetModFile;
use crate::response::{DataFuture, ResponseFuture};
use crate::types::files::File;
use crate::types::id::{FileId, GameId, ModId};
use crate::util::download::{Error, ErrorKind};

pin_project_lite::pin_project! {
    pub struct GetFile {
        game_id: GameId,
        mod_id: ModId,
        file_id: FileId,
        #[pin]
        future: Either<ResponseFuture<File>, DataFuture<File>>,
    }
}

impl GetFile {
    pub(crate) fn new(http: &Client, game_id: GameId, mod_id: ModId, file_id: FileId) -> Self {
        Self {
            game_id,
            mod_id,
            file_id,
            future: Either::Left(GetModFile::new(http, game_id, mod_id, file_id).into_future()),
        }
    }
}

impl Future for GetFile {
    type Output = Result<File, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        loop {
            match this.future.as_mut().as_pin_mut() {
                Either::Left(fut) => {
                    let resp = ready!(fut.poll(cx)).map_err(Error::request)?;

                    if resp.status() == StatusCode::NOT_FOUND {
                        let kind = ErrorKind::FileNotFound {
                            game_id: *this.game_id,
                            mod_id: *this.mod_id,
                            file_id: *this.file_id,
                        };
                        return Poll::Ready(Err(Error::new(kind)));
                    }
                    this.future.set(Either::Right(resp.data()));
                }
                Either::Right(fut) => {
                    return fut.poll(cx).map_err(Error::body);
                }
            }
        }
    }
}
