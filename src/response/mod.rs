//! `Response` type and related utility types.
//!
//! # Example
//!
//! ```no_run
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use modio::types::games::Game;
//! use modio::types::id::Id;
//! use modio::Client;
//!
//! let client = Client::builder(std::env::var("MODIO_API_KEY")?).build()?;
//!
//! let response = client.get_game(Id::new(51)).await?;
//! println!("http status: {}", response.status());
//!
//! let game: Game = response.data().await?;
//! println!("name: {}", game.name);
//! #     Ok(())
//! # }
//! ```

use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::Bytes;
use http_body_util::{BodyExt, Collected};
use serde::de::DeserializeOwned;

mod error;
mod future;

use crate::client;
use crate::error::Error;
use crate::types::ErrorResponse;

use self::error::BodyErrorKind;

pub use self::error::BodyError;
pub use self::future::ResponseFuture;

pub(crate) type Output<T> = Result<Response<T>, Error>;

/// Marker that the response has no content.
#[non_exhaustive]
pub struct NoContent;

/// A `Response` from a submitted request.
pub struct Response<T> {
    inner: client::service::Response,
    phantom: PhantomData<T>,
}

impl<T> Response<T> {
    pub(crate) const fn new(inner: client::service::Response) -> Self {
        Self {
            inner,
            phantom: PhantomData,
        }
    }

    /// Returns a reference to the response headers.
    pub fn headers(&self) -> &http::HeaderMap {
        self.inner.headers()
    }

    /// Returns the status code of the response.
    pub fn status(&self) -> http::StatusCode {
        self.inner.status()
    }

    /// Consumes the response and accumulate the body into bytes.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use bytes::Bytes;
    /// use modio::types::id::Id;
    /// use modio::Client;
    ///
    /// let client = Client::builder(std::env::var("MODIO_API_KEY")?).build()?;
    /// let response = client.get_game(Id::new(51)).await?;
    /// let bytes: Bytes = response.bytes().await?;
    /// #     Ok(())
    /// # }
    /// ```
    pub fn bytes(self) -> BytesFuture {
        let body = self.inner.into_body();

        let fut = async {
            body.collect()
                .await
                .map(Collected::to_bytes)
                .map_err(|err| BodyError::new(BodyErrorKind::Loading, Some(err)))
        };
        BytesFuture {
            inner: Box::pin(fut),
        }
    }

    /// Consumes the response and accumulate the body into a string.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use modio::types::id::Id;
    /// use modio::Client;
    ///
    /// let client = Client::builder(std::env::var("MODIO_API_KEY")?).build()?;
    /// let response = client.get_game(Id::new(51)).await?;
    /// let text: String = response.text().await?;
    /// #     Ok(())
    /// # }
    /// ```
    pub fn text(self) -> TextFuture {
        TextFuture::new(self.bytes())
    }

    fn error(self) -> ErrorResponseFuture {
        ErrorResponseFuture::new(self.bytes())
    }
}

impl<T: DeserializeOwned> Response<T> {
    /// Consume the response and deserialize the body into the request's matching model.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use modio::types::games::Game;
    /// use modio::types::id::Id;
    /// use modio::Client;
    ///
    /// let client = Client::builder(std::env::var("MODIO_API_KEY")?).build()?;
    /// let response = client.get_game(Id::new(51)).await?;
    /// let game: Game = response.data().await?;
    /// #     Ok(())
    /// # }
    /// ```
    pub fn data(self) -> DataFuture<T> {
        DataFuture::new(self.bytes())
    }
}

/// A `Future` that will resolve to the bytes of a response body.
///
/// This returned by [`Response::bytes`].
pub struct BytesFuture {
    inner: Pin<Box<dyn Future<Output = Result<Bytes, BodyError>> + Send + Sync>>,
}

impl Future for BytesFuture {
    type Output = Result<Bytes, BodyError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.inner.as_mut().poll(cx)
    }
}

/// A `Future` that will resolve to a deserialized model.
pub struct DataFuture<T> {
    inner: BytesFuture,
    phantom: PhantomData<T>,
}

impl<T> DataFuture<T> {
    const fn new(bytes: BytesFuture) -> Self {
        Self {
            inner: bytes,
            phantom: PhantomData,
        }
    }
}

impl<T: DeserializeOwned + Unpin> Future for DataFuture<T> {
    type Output = Result<T, BodyError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::new(&mut self.inner).poll(cx) {
            Poll::Ready(Ok(bytes)) => Poll::Ready(
                serde_json::from_slice(&bytes).map_err(|err| BodyError::decode(bytes, err)),
            ),
            Poll::Ready(Err(err)) => Poll::Ready(Err(err)),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// A `Future` that will resolve to the text of a response body.
///
/// This returned by [`Response::text`].
pub struct TextFuture {
    inner: BytesFuture,
}

impl TextFuture {
    const fn new(bytes: BytesFuture) -> Self {
        Self { inner: bytes }
    }
}

impl Future for TextFuture {
    type Output = Result<String, BodyError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::new(&mut self.inner).poll(cx) {
            Poll::Ready(Ok(bytes)) => {
                Poll::Ready(String::from_utf8(bytes.to_vec()).map_err(|err| {
                    let utf8_error = err.utf8_error();
                    BodyError::invalid_utf8(err.into_bytes(), utf8_error)
                }))
            }
            Poll::Ready(Err(err)) => Poll::Ready(Err(err)),
            Poll::Pending => Poll::Pending,
        }
    }
}

struct ErrorResponseFuture {
    inner: DataFuture<ErrorResponse>,
}

impl ErrorResponseFuture {
    const fn new(bytes: BytesFuture) -> Self {
        Self {
            inner: DataFuture::new(bytes),
        }
    }
}

impl Future for ErrorResponseFuture {
    type Output = Result<ErrorResponse, BodyError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner).poll(cx)
    }
}
