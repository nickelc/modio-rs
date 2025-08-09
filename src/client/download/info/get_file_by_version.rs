use std::future::{Future, IntoFuture};
use std::pin::Pin;
use std::task::{ready, Context, Poll};

use futures_util::future::Either;
use http::StatusCode;

use crate::client::download::{Error as DownloadError, ResolvePolicy};
use crate::client::Client;
use crate::error::{self, Error};
use crate::request::files::filters::Version;
use crate::request::files::GetFiles;
use crate::request::filter::prelude::*;
use crate::response::{DataFuture, ResponseFuture};
use crate::types::files::File;
use crate::types::id::{GameId, ModId};
use crate::types::List;

pin_project_lite::pin_project! {
    pub struct GetFileByVersion {
        game_id: GameId,
        mod_id: ModId,
        version: String,
        policy: ResolvePolicy,
        #[pin]
        future: Either<ResponseFuture<List<File>>, DataFuture<List<File>>>,
    }
}

impl GetFileByVersion {
    pub(crate) fn new(
        http: &Client,
        game_id: GameId,
        mod_id: ModId,
        version: String,
        policy: ResolvePolicy,
    ) -> Self {
        let filter = Version::eq(version.clone())
            .order_by(DateAdded::desc())
            .limit(2);

        let fut = GetFiles::new(http, game_id, mod_id)
            .filter(filter)
            .into_future();

        Self {
            game_id,
            mod_id,
            version,
            policy,
            future: Either::Left(fut),
        }
    }
}

impl Future for GetFileByVersion {
    type Output = Result<File, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        loop {
            match this.future.as_mut().as_pin_mut() {
                Either::Left(fut) => {
                    let resp = ready!(fut.poll(cx))?;

                    if resp.status() == StatusCode::NOT_FOUND {
                        let err = DownloadError::ModNotFound {
                            game_id: *this.game_id,
                            mod_id: *this.mod_id,
                        };
                        return Poll::Ready(Err(error::download(err)));
                    }

                    this.future.set(Either::Right(resp.data()));
                }
                Either::Right(fut) => {
                    let mut list = match fut.poll(cx) {
                        Poll::Ready(Ok(list)) => list.data,
                        Poll::Ready(Err(err)) => return Poll::Ready(Err(error::download(err))),
                        Poll::Pending => return Poll::Pending,
                    };

                    let result = match (list.len(), &this.policy) {
                        (1, _) | (_, ResolvePolicy::Latest) => Ok(list.remove(0)),
                        (0, _) => Err({
                            let err = DownloadError::VersionNotFound {
                                game_id: *this.game_id,
                                mod_id: *this.mod_id,
                                version: this.version.clone(),
                            };
                            error::download(err)
                        }),
                        (_, ResolvePolicy::Fail) => Err({
                            let err = DownloadError::MultipleFilesFound {
                                game_id: *this.game_id,
                                mod_id: *this.mod_id,
                                version: this.version.clone(),
                            };
                            error::download(err)
                        }),
                    };

                    return Poll::Ready(result);
                }
            }
        }
    }
}
