use std::future::{Future, IntoFuture};
use std::pin::Pin;
use std::task::{ready, Context, Poll};

use futures_util::future::Either;
use http::StatusCode;

use crate::client::Client;
use crate::request::mods::GetMod;
use crate::response::{DataFuture, ResponseFuture};
use crate::types::files::File;
use crate::types::id::{GameId, ModId};
use crate::types::mods::Mod;
use crate::util::download::{Error, ErrorKind};

pin_project_lite::pin_project! {
    pub struct GetPrimaryFile {
        game_id: GameId,
        mod_id: ModId,
        #[pin]
        future: Either<ResponseFuture<Mod>, DataFuture<Mod>>,
    }
}

impl GetPrimaryFile {
    pub(crate) fn new(http: &Client, game_id: GameId, mod_id: ModId) -> Self {
        Self {
            game_id,
            mod_id,
            future: Either::Left(GetMod::new(http, game_id, mod_id).into_future()),
        }
    }
}

impl Future for GetPrimaryFile {
    type Output = Result<File, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        loop {
            match this.future.as_mut().as_pin_mut() {
                Either::Left(fut) => {
                    let resp = ready!(fut.poll(cx)).map_err(Error::request)?;

                    if resp.status() == StatusCode::NOT_FOUND {
                        let kind = ErrorKind::ModNotFound {
                            game_id: *this.game_id,
                            mod_id: *this.mod_id,
                        };
                        return Poll::Ready(Err(Error::new(kind)));
                    }
                    this.future.set(Either::Right(resp.data()));
                }
                Either::Right(fut) => {
                    let mod_ = ready!(fut.poll(cx)).map_err(Error::body)?;

                    let Some(file) = mod_.modfile else {
                        let kind = ErrorKind::NoPrimaryFile {
                            game_id: *this.game_id,
                            mod_id: *this.mod_id,
                        };
                        return Poll::Ready(Err(Error::new(kind)));
                    };

                    return Poll::Ready(Ok(file));
                }
            }
        }
    }
}
