use std::marker::Unpin;
use std::path::Path;

use bytes::BytesMut;
use futures_util::{TryFutureExt, TryStreamExt};
use mime::Mime;
use reqwest::multipart::Part;
use reqwest::Body;
use tokio::codec::{BytesCodec, FramedRead};
use tokio::fs::File;
use tokio::io::AsyncRead;

pub struct FileSource {
    pub body: Body,
    pub filename: String,
    pub mime: Mime,
}

impl FileSource {
    pub fn new_from_file<P: AsRef<Path>>(file: P, filename: String, mime: Mime) -> Self {
        let file = file.as_ref().to_path_buf();
        let st = File::open(file)
            .map_ok(|file| FramedRead::new(file, BytesCodec::new()).map_ok(BytesMut::freeze))
            .try_flatten_stream();

        FileSource {
            body: Body::wrap_stream(st),
            filename,
            mime,
        }
    }

    pub fn new_from_read<T>(read: T, filename: String, mime: Mime) -> Self
    where
        T: AsyncRead + Send + Sync + Unpin + 'static,
    {
        let st = FramedRead::new(read, BytesCodec::new()).map_ok(BytesMut::freeze);

        FileSource {
            body: Body::wrap_stream(st),
            filename,
            mime,
        }
    }
}

impl From<FileSource> for Part {
    fn from(source: FileSource) -> Part {
        Part::stream(source.body)
            .file_name(source.filename)
            .mime_str(&source.mime.to_string())
            .expect("FileSource::into::<Part>()")
    }
}
