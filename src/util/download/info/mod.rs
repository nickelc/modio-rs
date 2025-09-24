use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{ready, Context, Poll};

use pin_project_lite::pin_project;
use url::Url;

use crate::types::files::File;
use crate::types::id::FileId;
use crate::Client;

use super::{DownloadAction, Error};

mod get_file;
mod get_file_by_version;
mod get_primary_file;

use self::get_file::GetFile;
use self::get_file_by_version::GetFileByVersion;
use self::get_primary_file::GetPrimaryFile;

#[non_exhaustive]
pub struct Info {
    pub file_id: FileId,
    pub download_url: Url,
    pub filesize: u64,
    pub filesize_uncompressed: u64,
    pub filehash: String,
}

impl fmt::Debug for Info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Info")
            .field("file_id", &self.file_id)
            .field("download_url", &self.download_url.as_str())
            .field("filesize", &self.filesize)
            .field("filesize_uncompressed", &self.filesize_uncompressed)
            .field("filehash", &self.filehash)
            .finish_non_exhaustive()
    }
}

pin_project! {
    pub struct GetInfo {
        #[pin]
        future: FileFuture,
    }
}

pin_project! {
    #[project = FileFutureProj]
    enum FileFuture {
        Primary {
            #[pin]
            future: GetPrimaryFile,
        },
        File {
            #[pin]
            future: GetFile,
        },
        FileObj {
            file: Option<Box<File>>,
        },
        Version {
            #[pin]
            future: GetFileByVersion,
        }
    }
}

impl Future for FileFuture {
    type Output = Result<File, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.project() {
            FileFutureProj::Primary { future } => future.poll(cx),
            FileFutureProj::File { future } => future.poll(cx),
            FileFutureProj::FileObj { file } => {
                Poll::Ready(Ok(*file.take().expect("polled after completion")))
            }
            FileFutureProj::Version { future } => future.poll(cx),
        }
    }
}

impl GetInfo {
    pub(crate) fn new(http: &Client, action: DownloadAction) -> Self {
        let future = match action {
            DownloadAction::Primary { game_id, mod_id } => FileFuture::Primary {
                future: GetPrimaryFile::new(http, game_id, mod_id),
            },
            DownloadAction::File {
                game_id,
                mod_id,
                file_id,
            } => FileFuture::File {
                future: GetFile::new(http, game_id, mod_id, file_id),
            },
            DownloadAction::FileObj(file) => FileFuture::FileObj { file: Some(file) },
            DownloadAction::Version {
                game_id,
                mod_id,
                version,
                policy,
            } => FileFuture::Version {
                future: GetFileByVersion::new(http, game_id, mod_id, version, policy),
            },
        };
        Self { future }
    }
}

impl Future for GetInfo {
    type Output = Result<Info, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let file = ready!(self.project().future.poll(cx))?;

        Poll::Ready(Ok(Info {
            file_id: file.id,
            download_url: file.download.binary_url,
            filesize: file.filesize,
            filesize_uncompressed: file.filesize_uncompressed,
            filehash: file.filehash.md5,
        }))
    }
}

pub fn download_info(http: &Client, action: DownloadAction) -> GetInfo {
    GetInfo::new(http, action)
}
