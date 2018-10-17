use std::io::Error;
use std::path::{Path, PathBuf};

use bytes::Bytes;
use futures::{task, Async, Future, Poll, Stream};
use mime::Mime;
use mpart_async::MultipartRequest;
use tokio_codec::{BytesCodec, FramedRead};
use tokio_fs::file::{File, OpenFuture};
use tokio_io::AsyncRead;

pub type MultipartForm = MultipartRequest<FileStream>;

pub struct FileSource {
    pub inner: FileStream,
    pub filename: String,
    pub mime: Mime,
}

enum State {
    File(OpenFuture<PathBuf>),
    Read(FramedRead<Box<AsyncRead + Send + Sync>, BytesCodec>),
}

pub struct FileStream {
    state: Option<State>,
}

impl FileStream {
    pub fn new<T: 'static + AsyncRead + Send + Sync>(inner: T) -> FileStream {
        let framed = FramedRead::new(
            Box::new(inner) as Box<AsyncRead + Send + Sync>,
            BytesCodec::new(),
        );
        FileStream {
            state: Some(State::Read(framed)),
        }
    }

    pub fn open<P: AsRef<Path>>(file: P) -> FileStream {
        FileStream {
            state: Some(State::File(File::open(file.as_ref().to_path_buf()))),
        }
    }
}

impl Stream for FileStream {
    type Item = Bytes;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.state.take() {
            Some(State::File(mut stream)) => {
                if let Async::Ready(file) = stream.poll()? {
                    let framed = FramedRead::new(
                        Box::new(file) as Box<AsyncRead + Send + Sync>,
                        BytesCodec::new(),
                    );
                    self.state = Some(State::Read(framed));
                    task::current().notify();
                } else {
                    self.state = Some(State::File(stream));
                }
                Ok(Async::NotReady)
            }
            Some(State::Read(mut stream)) => {
                let ret = stream.poll();
                self.state = Some(State::Read(stream));
                if let Async::Ready(bytes) = ret? {
                    Ok(Async::Ready(bytes.map(|b| b.freeze())))
                } else {
                    Ok(Async::NotReady)
                }
            }
            None => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate tokio;

    use super::*;
    use std::io;

    use self::tokio::runtime::Runtime;
    use futures::{Future, Stream};

    #[test]
    fn new() {
        let mut rt = Runtime::new().expect("new rt");

        let r = io::Cursor::new(b"Hello World");
        let fs = FileStream::new(r).concat2().and_then(|bytes| {
            assert_eq!(bytes, &b"Hello World"[..]);
            Ok(())
        });

        rt.block_on(fs).unwrap();
    }

    #[test]
    fn open() {
        let mut rt = Runtime::new().expect("new rt");

        let fs = FileStream::open("Cargo.toml").concat2().and_then(|bytes| {
            assert_eq!(bytes, &include_bytes!("../Cargo.toml")[..]);
            Ok(())
        });

        rt.block_on(fs).unwrap();
    }
}
