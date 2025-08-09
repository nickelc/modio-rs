use std::borrow::Cow;
use std::ffi::OsStr;
use std::future;
use std::path::Path;
use std::pin::Pin;
use std::task::{self, Context, Poll};

use bytes::{Bytes, BytesMut};
use futures_util::future::{Either, TryFutureExt};
use futures_util::stream::{self, Stream, StreamExt, TryStream, TryStreamExt};
use http::header::HeaderMap;
use mime::Mime;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

mod boundary;
mod encoding;

use boundary::Boundary;

const CONTENT_TYPE: &[u8] = b"multipart/form-data; boundary=";
const CONTENT_DISPOSITION: &[u8] = b"Content-Disposition: form-data; ";

type Error = Box<dyn std::error::Error + Send + Sync>;
type BoxStream = Pin<Box<dyn Stream<Item = Result<Bytes, Error>> + Send + Sync>>;

pub struct Form {
    inner: Parts,
}

pub struct Part {
    meta: Metadata,
    value: BodyStream,
    body_length: Option<u64>,
}

struct Parts {
    boundary: Boundary,
    fields: Vec<(Cow<'static, str>, Part)>,
    percent_encoding: PercentEncoding,
}

struct Metadata {
    mime: Option<Mime>,
    file_name: Option<Cow<'static, str>>,
    headers: HeaderMap,
}

pub struct BodyStream {
    inner: Inner,
}

enum Inner {
    Bytes(Bytes),
    Stream(BoxStream),
}

pin_project_lite::pin_project! {
    struct WrapStream<S> {
        #[pin]
        inner: S,
    }
}

impl Form {
    pub fn new() -> Self {
        Self {
            inner: Parts::new(),
        }
    }

    pub fn content_type(&self) -> Bytes {
        let mut buf = BytesMut::with_capacity(CONTENT_TYPE.len() + Boundary::LEN);
        buf.extend_from_slice(CONTENT_TYPE);
        buf.extend_from_slice(self.boundary());
        buf.freeze()
    }

    pub fn boundary(&self) -> &[u8] {
        self.inner.boundary()
    }

    pub fn text<T, U>(self, name: T, value: U) -> Self
    where
        T: Into<Cow<'static, str>>,
        U: Into<Cow<'static, str>>,
    {
        self.part(name, Part::text(value))
    }

    /// Adds a customized Part.
    pub fn part<T>(self, name: T, part: Part) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        self.with_inner(|inner| inner.part(name, part))
    }

    #[allow(dead_code)]
    pub fn percent_encode_path_seqment(self) -> Self {
        self.with_inner(Parts::percent_encode_path_seqment)
    }

    #[allow(dead_code)]
    pub fn percent_encode_attr_char(self) -> Self {
        self.with_inner(Parts::percent_encode_attr_char)
    }

    #[allow(dead_code)]
    pub fn percent_encode_noop(self) -> Self {
        self.with_inner(Parts::percent_encode_noop)
    }

    pub fn stream(mut self) -> impl Stream<Item = Result<Bytes, Error>> {
        if self.inner.fields.is_empty() {
            return Either::Left(stream::empty());
        }

        let (name, part) = self.inner.fields.remove(0);
        let start = Box::pin(self.part_stream(name, part))
            as Pin<Box<dyn Stream<Item = Result<Bytes, Error>> + Send + Sync>>;

        let fields = std::mem::take(&mut self.inner.fields);
        let stream = fields.into_iter().fold(start, |memo, (name, part)| {
            let stream = self.part_stream(name, part);
            Box::pin(memo.chain(stream))
                as Pin<Box<dyn Stream<Item = Result<Bytes, Error>> + Send + Sync>>
        });

        let last = stream::once(future::ready(Ok(self.inner.boundary.terminator())));

        Either::Right(stream.chain(last))
    }

    fn part_stream<T>(&mut self, name: T, part: Part) -> impl Stream<Item = Result<Bytes, Error>>
    where
        T: Into<Cow<'static, str>>,
    {
        let boundary = stream::once(future::ready(Ok(self.inner.boundary.delimiter())));
        let header = stream::once(future::ready(Ok({
            let mut h = self
                .inner
                .percent_encoding
                .encode_headers(&name.into(), &part.meta);
            h.extend_from_slice(b"\r\n\r\n");
            h.into()
        })));
        boundary
            .chain(header)
            .chain(part.value)
            .chain(stream::once(future::ready(Ok("\r\n".into()))))
    }

    pub fn compute_length(&self) -> Option<u64> {
        self.inner.compute_length()
    }

    fn with_inner<F>(self, func: F) -> Self
    where
        F: FnOnce(Parts) -> Parts,
    {
        Form {
            inner: func(self.inner),
        }
    }
}

impl Part {
    fn new(value: BodyStream, body_length: Option<u64>) -> Self {
        Self {
            meta: Metadata::new(),
            value,
            body_length,
        }
    }

    pub fn text<T>(value: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        let value = match value.into() {
            Cow::Borrowed(slice) => Bytes::from(slice),
            Cow::Owned(string) => Bytes::from(string),
        };
        Self::bytes(value)
    }

    pub fn bytes(value: Bytes) -> Self {
        Self::new(BodyStream::bytes(value), None)
    }

    pub fn stream<S>(value: S) -> Self
    where
        S: TryStream + Send + Sync + 'static,
        S::Error: Into<Error>,
        Bytes: From<S::Ok>,
    {
        Self::new(BodyStream::stream(value), None)
    }

    #[allow(dead_code)]
    pub fn stream_with_length<S>(value: S, length: u64) -> Self
    where
        S: TryStream + Send + Sync + 'static,
        S::Error: Into<Error>,
        Bytes: From<S::Ok>,
    {
        Self::new(BodyStream::stream(value), Some(length))
    }

    pub fn file<P: AsRef<Path>>(file: P, filename: &str) -> Self {
        let file = file.as_ref();
        let filename = file
            .file_name()
            .and_then(OsStr::to_str)
            .map_or_else(|| filename.to_owned(), ToOwned::to_owned);

        let stream = File::open(file.to_path_buf())
            .map_ok(ReaderStream::new)
            .try_flatten_stream();

        Self::stream(stream).file_name(filename)
    }

    pub fn mime(self, mime: Mime) -> Self {
        self.with_inner(|inner| inner.mime(mime))
    }

    pub fn file_name<T>(self, filename: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        self.with_inner(|inner| inner.file_name(filename))
    }

    fn value_length(&self) -> Option<u64> {
        self.body_length.or_else(|| {
            let (_, upper) = self.value.size_hint();
            upper.map(|v| v as u64)
        })
    }

    fn with_inner<F>(self, func: F) -> Self
    where
        F: FnOnce(Metadata) -> Metadata,
    {
        Part {
            meta: func(self.meta),
            ..self
        }
    }
}

impl Parts {
    fn new() -> Self {
        Self {
            boundary: Boundary::generate(),
            fields: Vec::new(),
            percent_encoding: PercentEncoding::PathSegment,
        }
    }

    fn boundary(&self) -> &[u8] {
        self.boundary.value()
    }

    fn part<T>(mut self, name: T, part: Part) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        self.fields.push((name.into(), part));
        self
    }

    fn percent_encode_path_seqment(mut self) -> Self {
        self.percent_encoding = PercentEncoding::PathSegment;
        self
    }

    fn percent_encode_attr_char(mut self) -> Self {
        self.percent_encoding = PercentEncoding::AttrChar;
        self
    }

    fn percent_encode_noop(mut self) -> Self {
        self.percent_encoding = PercentEncoding::NoOp;
        self
    }

    fn compute_length(&self) -> Option<u64> {
        let mut length = 0;

        for (name, field) in &self.fields {
            match field.value_length() {
                Some(value_length) => {
                    let header = self.percent_encoding.encode_headers(name, &field.meta);
                    let header_length = header.len() as u64;
                    length += 2 + Boundary::LEN as u64 + 2 + header_length + 4 + value_length + 2;
                }
                _ => return None,
            }
        }
        if !self.fields.is_empty() {
            length += 2 + self.boundary().len() as u64 + 4;
        }
        Some(length)
    }
}

impl Metadata {
    fn new() -> Self {
        Self {
            file_name: None,
            mime: None,
            headers: HeaderMap::default(),
        }
    }

    fn mime(mut self, mime: Mime) -> Self {
        self.mime = Some(mime);
        self
    }

    fn file_name<T>(mut self, filename: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        self.file_name = Some(filename.into());
        self
    }
}

impl BodyStream {
    pub fn bytes(value: Bytes) -> Self {
        Self {
            inner: Inner::Bytes(value),
        }
    }

    pub fn stream<S>(value: S) -> Self
    where
        S: TryStream + Send + Sync + 'static,
        S::Error: Into<Error>,
        Bytes: From<S::Ok>,
    {
        let body = Box::pin(WrapStream {
            inner: value.map_ok(Bytes::from).map_err(Into::into),
        });
        Self {
            inner: Inner::Stream(body),
        }
    }
}

impl Stream for BodyStream {
    type Item = Result<Bytes, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.inner {
            Inner::Bytes(ref mut bytes) => {
                if bytes.is_empty() {
                    Poll::Ready(None)
                } else {
                    Poll::Ready(Some(Ok(std::mem::replace(bytes, Bytes::new()))))
                }
            }
            Inner::Stream(ref mut stream) => {
                let chunk = task::ready!(Pin::new(stream).poll_next(cx));
                Poll::Ready(chunk)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match &self.inner {
            Inner::Bytes(bytes) => {
                let len = bytes.len();
                (len, Some(len))
            }
            Inner::Stream(stream) => stream.size_hint(),
        }
    }
}

impl<S, D, E> Stream for WrapStream<S>
where
    S: Stream<Item = Result<D, E>>,
    Bytes: From<D>,
    E: Into<Error>,
{
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let item = task::ready!(self.project().inner.poll_next(cx)?);
        Poll::Ready(item.map(Ok))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

enum PercentEncoding {
    PathSegment,
    AttrChar,
    NoOp,
}

impl PercentEncoding {
    fn encode_headers(&self, name: &str, field: &Metadata) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(CONTENT_DISPOSITION);

        let encode = match self {
            Self::PathSegment => encoding::percent_encode_path_segment,
            Self::AttrChar => encoding::percent_encode_attr_char,
            Self::NoOp => encoding::percent_encode_noop,
        };

        match encode(name) {
            Cow::Borrowed(value) => {
                buf.extend_from_slice(b"name=\"");
                buf.extend_from_slice(value.as_bytes());
                buf.extend_from_slice(b"\"");
            }
            Cow::Owned(value) => {
                buf.extend_from_slice(b"name*=utf8''");
                buf.extend_from_slice(value.as_bytes());
            }
        }

        if let Some(filename) = &field.file_name {
            buf.extend_from_slice(b"; filename=\"");
            let legal_filename = filename
                .replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\r', "\\\r")
                .replace('\n', "\\\n");
            buf.extend_from_slice(legal_filename.as_bytes());
            buf.extend_from_slice(b"\"");
        }
        if let Some(mime) = &field.mime {
            buf.extend_from_slice(b"\r\nContent-type: ");
            buf.extend_from_slice(mime.as_ref().as_bytes());
        }

        for (k, v) in &field.headers {
            buf.extend_from_slice(b"\r\n");
            buf.extend_from_slice(k.as_str().as_bytes());
            buf.extend_from_slice(b": ");
            buf.extend_from_slice(v.as_bytes());
        }
        buf
    }
}
