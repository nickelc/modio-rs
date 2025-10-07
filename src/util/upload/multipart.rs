use std::future::{Future, IntoFuture};
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{ready, Context, Poll};

use bytes::Bytes;
use futures_util::TryStream;

use crate::request::files::multipart::{
    AddMultipartUploadFile, ContentRange, CreateMultipartUploadSession,
};
use crate::types::files::multipart::{UploadId, UploadPart, UploadSession};
use crate::types::files::File;
use crate::types::id::{GameId, ModId};
use crate::util::{DataError, DataFromRequest, DataFuture};
use crate::Client;

use super::Error;

pub struct MultipartUploader<'a, State> {
    state: State,
    phantom: PhantomData<fn(&'a State) -> State>,
}

impl<'a> MultipartUploader<'a, ()> {
    pub(crate) const fn init(
        http: &'a Client,
        game_id: GameId,
        mod_id: ModId,
        filename: &'a str,
    ) -> MultipartUploader<'a, Init<'a>> {
        MultipartUploader {
            state: Init {
                http,
                game_id,
                mod_id,
                create_session: http.create_multipart_upload_session(game_id, mod_id, filename),
            },
            phantom: PhantomData,
        }
    }

    pub(crate) const fn started(
        http: &'a Client,
        game_id: GameId,
        mod_id: ModId,
        upload_id: UploadId,
    ) -> MultipartUploader<'a, Started<'a>> {
        MultipartUploader {
            state: Started {
                http,
                game_id,
                mod_id,
                upload_id,
            },
            phantom: PhantomData,
        }
    }
}

pub struct Init<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    create_session: CreateMultipartUploadSession<'a>,
}

impl<'a> MultipartUploader<'a, Init<'a>> {
    /// An optional nonce to provide to prevent duplicate upload sessions
    /// from being created concurrently.
    ///
    /// Maximum 64 characters (Recommended: SHA-256)
    pub const fn nonce(mut self, nonce: &'a str) -> Self {
        self.state.create_session = self.state.create_session.nonce(nonce);
        self
    }
}

impl<'a> IntoFuture for MultipartUploader<'a, Init<'a>> {
    type Output = <MultipartUploaderInitFuture<'a> as Future>::Output;
    type IntoFuture = MultipartUploaderInitFuture<'a>;

    fn into_future(self) -> Self::IntoFuture {
        let Init {
            http,
            game_id,
            mod_id,
            create_session,
        } = self.state;

        MultipartUploaderInitFuture {
            future: create_session.data(),
            state: Some((http, game_id, mod_id)),
        }
    }
}

pin_project_lite::pin_project! {
    pub struct MultipartUploaderInitFuture<'a> {
        #[pin]
        future: DataFuture<UploadSession>,
        state: Option<(&'a Client, GameId, ModId)>,
    }
}

impl<'a> Future for MultipartUploaderInitFuture<'a> {
    type Output = Result<MultipartUploader<'a, Started<'a>>, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();

        let result = match ready!(this.future.as_mut().poll(cx)) {
            Ok(session) => {
                let (http, game_id, mod_id) = this.state.take().expect("Polled after completion");
                Ok(MultipartUploader {
                    state: Started {
                        http,
                        game_id,
                        mod_id,
                        upload_id: session.id,
                    },
                    phantom: PhantomData,
                })
            }
            Err(err) => Err(match err {
                DataError::Request(err) => Error::request(err),
                DataError::Body(err) => Error::body(err),
            }),
        };

        Poll::Ready(result)
    }
}

pub struct Started<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    upload_id: UploadId,
}

impl<'a> MultipartUploader<'a, Started<'a>> {
    pub const fn id(&self) -> UploadId {
        self.state.upload_id
    }

    /// Get the uploaded parts of the current session.
    pub async fn get_parts(&self) -> Result<Vec<UploadPart>, Error> {
        let Started {
            http,
            game_id,
            mod_id,
            upload_id,
        } = self.state;

        match http
            .get_multipart_upload_parts(game_id, mod_id, upload_id)
            .data()
            .await
        {
            Ok(parts) => Ok(parts.data),
            Err(err) => Err(match err {
                DataError::Request(err) => Error::request(err),
                DataError::Body(err) => Error::body(err),
            }),
        }
    }

    /// Add a new part to the upload session.
    pub async fn add_part<S>(&self, range: ContentRange, stream: S) -> Result<UploadPart, Error>
    where
        S: TryStream + Send + 'static,
        S::Ok: Into<Bytes>,
        S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        let Started {
            http,
            game_id,
            mod_id,
            upload_id,
        } = self.state;

        match http
            .add_multipart_upload_part(game_id, mod_id, upload_id, range, stream)
            .data()
            .await
        {
            Ok(part) => Ok(part),
            Err(err) => Err(match err {
                DataError::Request(err) => Error::request(err),
                DataError::Body(err) => Error::body(err),
            }),
        }
    }

    /// Complete the active upload session after uploading all parts with
    /// [`MultipartUploader::add_part`].
    pub async fn complete(self) -> Result<MultipartUploader<'a, Completed<'a>>, Error> {
        let Started {
            http,
            game_id,
            mod_id,
            upload_id,
        } = self.state;

        let upload_id = match http
            .complete_multipart_upload_session(game_id, mod_id, upload_id)
            .data()
            .await
        {
            Ok(UploadSession { id, .. }) => id,
            Err(err) => {
                return Err(match err {
                    DataError::Request(err) => Error::request(err),
                    DataError::Body(err) => Error::body(err),
                })
            }
        };

        let add_file = http.add_multipart_upload_file(game_id, mod_id, upload_id);
        Ok(MultipartUploader {
            state: Completed { add_file },
            phantom: PhantomData,
        })
    }

    /// Terminate the upload session.
    pub async fn abort(self) -> Result<(), Error> {
        let Started {
            http,
            game_id,
            mod_id,
            upload_id,
        } = self.state;

        if let Err(err) = http
            .delete_multipart_upload_session(game_id, mod_id, upload_id)
            .await
        {
            return Err(Error::request(err));
        }

        Ok(())
    }
}

pub struct Completed<'a> {
    add_file: AddMultipartUploadFile<'a>,
}

impl<'a> MultipartUploader<'a, Completed<'a>> {
    pub const fn active(mut self, active: bool) -> Self {
        self.state.add_file = self.state.add_file.active(active);
        self
    }

    pub const fn changelog(mut self, changelog: &'a str) -> Self {
        self.state.add_file = self.state.add_file.changelog(changelog);
        self
    }

    pub const fn filehash(mut self, filehash: &'a str) -> Self {
        self.state.add_file = self.state.add_file.filehash(filehash);
        self
    }

    pub const fn metadata_blob(mut self, metadata: &'a str) -> Self {
        self.state.add_file = self.state.add_file.metadata_blob(metadata);
        self
    }

    pub const fn version(mut self, version: &'a str) -> Self {
        self.state.add_file = self.state.add_file.version(version);
        self
    }
}

impl IntoFuture for MultipartUploader<'_, Completed<'_>> {
    type Output = <MultipartUploaderCompleteFuture as Future>::Output;
    type IntoFuture = MultipartUploaderCompleteFuture;

    fn into_future(self) -> Self::IntoFuture {
        let Completed { add_file } = self.state;

        MultipartUploaderCompleteFuture {
            future: add_file.data(),
        }
    }
}

pin_project_lite::pin_project! {
    pub struct MultipartUploaderCompleteFuture {
        #[pin]
        future: DataFuture<File>,
    }
}

impl Future for MultipartUploaderCompleteFuture {
    type Output = Result<File, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();

        let result = match ready!(this.future.as_mut().poll(cx)) {
            Ok(file) => Ok(file),
            Err(err) => Err(match err {
                DataError::Request(err) => Error::request(err),
                DataError::Body(err) => Error::body(err),
            }),
        };

        Poll::Ready(result)
    }
}
