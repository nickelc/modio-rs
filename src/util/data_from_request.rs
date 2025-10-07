use std::fmt;
use std::future::{Future, IntoFuture};
use std::pin::Pin;
use std::task::{ready, Context, Poll};

use futures_util::future::Either;
use serde::de::DeserializeOwned;

use crate::response::{self, BodyError, ResponseFuture};
use crate::Error;

/// Extension trait for typed request builder objects.
///
/// Allows the user to retrieve the deserialized model directly from the request builder without
/// going through the `Response<T>` object.
pub trait DataFromRequest<T: DeserializeOwned + Unpin>: private::Sealed {
    /// Consume the request builder and deserialize the body into the request's matching model.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #     let client = modio::Client::builder(std::env::var("MODIO_API_KEY")?).build()?;
    /// use modio::types::id::Id;
    /// use modio::types::mods::Mod;
    /// use modio::util::DataFromRequest;
    ///
    /// let mod_: Mod = client.get_mod(Id::new(51), Id::new(123)).data().await?;
    /// println!("name: {}", mod_.name);
    /// #     Ok(())
    /// # }
    /// ```
    fn data(self) -> DataFuture<T>;
}

/// The errors that may occur when using [`DataFromRequest::data()`].
#[derive(Debug)]
pub enum DataError {
    Request(Error),
    Body(BodyError),
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Request(err) => err.fmt(f),
            Self::Body(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for DataError {}

pin_project_lite::pin_project! {
    /// A `Future` that will resolve to a deserialized model.
    pub struct DataFuture<T> where T: Unpin {
        #[pin]
        future: Either<ResponseFuture<T>, response::DataFuture<T>>,
    }
}

impl<Builder, Data> DataFromRequest<Data> for Builder
where
    Builder: IntoFuture<IntoFuture = ResponseFuture<Data>> + private::Sealed,
    Data: DeserializeOwned + Unpin,
{
    fn data(self) -> DataFuture<Data> {
        DataFuture {
            future: Either::Left(self.into_future()),
        }
    }
}

impl<T: DeserializeOwned + Unpin> Future for DataFuture<T> {
    type Output = Result<T, DataError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        loop {
            match this.future.as_mut().as_pin_mut() {
                Either::Left(fut) => {
                    let resp = ready!(fut.poll(cx).map_err(DataError::Request))?;
                    this.future.set(Either::Right(resp.data()));
                }
                Either::Right(fut) => {
                    return fut.poll(cx).map_err(DataError::Body);
                }
            }
        }
    }
}

mod private {
    use crate::request::{auth, files, games, mods, user};

    pub trait Sealed {}

    impl Sealed for auth::EmailRequest<'_> {}
    impl Sealed for auth::EmailExchange<'_> {}
    impl<T> Sealed for auth::ExternalAuth<'_, T> {}
    impl Sealed for auth::GetTerms<'_> {}

    impl Sealed for games::AddGameMedia<'_> {}
    impl Sealed for games::GetGame<'_> {}
    impl Sealed for games::GetGameStats<'_> {}
    impl Sealed for games::GetGames<'_> {}
    impl Sealed for games::tags::AddGameTags<'_> {}
    impl Sealed for games::tags::GetGameTags<'_> {}
    impl Sealed for games::tags::RenameGameTag<'_> {}

    impl Sealed for mods::AddMod<'_> {}
    impl Sealed for mods::EditMod<'_> {}
    impl Sealed for mods::GetMod<'_> {}
    impl Sealed for mods::GetModTeamMembers<'_> {}
    impl Sealed for mods::GetMods<'_> {}
    impl Sealed for mods::SubmitModRating<'_> {}
    impl Sealed for mods::comments::AddModComment<'_> {}
    impl Sealed for mods::comments::EditModComment<'_> {}
    impl Sealed for mods::comments::GetModComment<'_> {}
    impl Sealed for mods::comments::GetModComments<'_> {}
    impl Sealed for mods::comments::UpdateModCommentKarma<'_> {}
    impl Sealed for mods::dependencies::AddModDependencies<'_> {}
    impl Sealed for mods::dependencies::GetModDependencies<'_> {}
    impl Sealed for mods::events::GetModEvents<'_> {}
    impl Sealed for mods::events::GetModsEvents<'_> {}
    impl Sealed for mods::media::AddModMedia<'_> {}
    impl Sealed for mods::metadata::AddModMetadata<'_> {}
    impl Sealed for mods::metadata::GetModMetadata<'_> {}
    impl Sealed for mods::stats::GetModStats<'_> {}
    impl Sealed for mods::stats::GetModsStats<'_> {}
    impl Sealed for mods::subscribe::SubscribeToMod<'_> {}
    impl Sealed for mods::tags::AddModTags<'_> {}
    impl Sealed for mods::tags::GetModTags<'_> {}

    impl Sealed for files::AddFile<'_> {}
    impl Sealed for files::EditFile<'_> {}
    impl Sealed for files::GetFile<'_> {}
    impl Sealed for files::GetFiles<'_> {}
    impl Sealed for files::ManagePlatformStatus<'_> {}
    impl Sealed for files::multipart::AddMultipartUploadFile<'_> {}
    impl<S> Sealed for files::multipart::AddMultipartUploadPart<'_, S> {}
    impl Sealed for files::multipart::CompleteMultipartUploadSession<'_> {}
    impl Sealed for files::multipart::CreateMultipartUploadSession<'_> {}
    impl Sealed for files::multipart::GetMultipartUploadParts<'_> {}
    impl Sealed for files::multipart::GetMultipartUploadSessions<'_> {}

    impl Sealed for user::GetAuthenticatedUser<'_> {}
    impl Sealed for user::GetMutedUsers<'_> {}
    impl Sealed for user::GetUserEvents<'_> {}
    impl Sealed for user::GetUserFiles<'_> {}
    impl Sealed for user::GetUserGames<'_> {}
    impl Sealed for user::GetUserMods<'_> {}
    impl Sealed for user::GetUserRatings<'_> {}
    impl Sealed for user::GetUserSubscriptions<'_> {}
}
