use std::pin::Pin;
use std::task::{ready, Context, Poll};

use bytes::Bytes;
use futures_util::TryStream;
use http_body::Frame;
use http_body_util::BodyExt;

type Error = Box<dyn std::error::Error + Send + Sync>;

type BoxBody = http_body_util::combinators::UnsyncBoxBody<Bytes, Error>;

fn boxed<B>(body: B) -> BoxBody
where
    B: http_body::Body<Data = Bytes> + Send + 'static,
    B::Error: Into<Error>,
{
    try_downcast(body).unwrap_or_else(|body| body.map_err(Into::into).boxed_unsync())
}

fn try_downcast<T, K>(k: K) -> Result<T, K>
where
    T: 'static,
    K: Send + 'static,
{
    let mut k = Some(k);
    if let Some(k) = <dyn std::any::Any>::downcast_mut::<Option<T>>(&mut k) {
        Ok(k.take().unwrap())
    } else {
        Err(k.unwrap())
    }
}

#[derive(Debug)]
pub struct Body(BoxBody);

impl Body {
    pub fn new<B>(body: B) -> Self
    where
        B: http_body::Body<Data = Bytes> + Send + 'static,
        B::Error: Into<Error>,
    {
        try_downcast(body).unwrap_or_else(|body| Self(boxed(body)))
    }

    pub fn empty() -> Self {
        Self::new(http_body_util::Empty::new())
    }

    pub fn from_stream<S>(stream: S) -> Self
    where
        S: TryStream + Send + 'static,
        S::Ok: Into<Bytes>,
        S::Error: Into<Error>,
    {
        Self::new(StreamBody { stream })
    }
}

impl Default for Body {
    fn default() -> Self {
        Self::empty()
    }
}

impl From<String> for Body {
    fn from(value: String) -> Self {
        Self::new(http_body_util::Full::from(value))
    }
}

impl http_body::Body for Body {
    type Data = Bytes;
    type Error = Error;

    #[inline]
    fn poll_frame(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        Pin::new(&mut self.0).poll_frame(cx)
    }

    #[inline]
    fn size_hint(&self) -> http_body::SizeHint {
        self.0.size_hint()
    }

    #[inline]
    fn is_end_stream(&self) -> bool {
        self.0.is_end_stream()
    }
}

pin_project_lite::pin_project! {
    struct StreamBody<S> {
        #[pin]
        stream: S,
    }
}

impl<S> http_body::Body for StreamBody<S>
where
    S: TryStream,
    S::Ok: Into<Bytes>,
    S::Error: Into<Error>,
{
    type Data = Bytes;
    type Error = Error;

    fn poll_frame(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        let this = self.as_mut().project();

        match ready!(this.stream.try_poll_next(cx)) {
            Some(Ok(chunk)) => Poll::Ready(Some(Ok(Frame::data(chunk.into())))),
            Some(Err(err)) => Poll::Ready(Some(Err(err.into()))),
            None => Poll::Ready(None),
        }
    }
}
