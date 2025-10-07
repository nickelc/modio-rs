use std::fmt;
use std::marker::PhantomData;
use std::vec::IntoIter;

use serde::de::DeserializeOwned;

use crate::request::{Filter, RequestBuilder, Route};
use crate::response::BodyError;
use crate::types::List;
use crate::{Client, Error};

/// Extension trait for typed request builder objects for [`List<T>`] responses.
pub trait Paginate<'a>: private::Sealed {
    type Output;

    /// # Examples
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #     let client = modio::Client::builder(std::env::var("MODIO_API_KEY")?).build()?;
    /// use modio::types::id::Id;
    /// use modio::types::mods::Mod;
    /// use modio::util::Paginate;
    ///
    /// let mods = client.get_mods(Id::new(51));
    /// let mut paged = mods.paged();
    ///
    /// while let Some(page) = paged.next().await? {
    ///     for mod_ in page {
    ///         println!("name: {}", mod_.name);
    ///     }
    /// }
    /// #     Ok(())
    /// # }
    /// ```
    fn paged(&'a self) -> Paginator<'a, Self::Output>;
}

pub struct Paginator<'a, T> {
    http: &'a Client,
    route: Route,
    filter: Filter,
    state: State,
    phantom: PhantomData<T>,
}

/// The errors that may occur when using [`Paginator::next()`].
#[derive(Debug)]
pub enum PaginateError {
    Request(Error),
    Body(BodyError),
}

impl fmt::Display for PaginateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Request(err) => err.fmt(f),
            Self::Body(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for PaginateError {}

#[derive(Debug)]
pub struct Page<T>(List<T>);

impl<T> std::ops::Deref for Page<T> {
    type Target = List<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> IntoIterator for Page<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.data.into_iter()
    }
}

enum State {
    Start,
    Next { offset: u32, limit: u32 },
    Completed,
}

impl<'a, T: DeserializeOwned + Unpin> Paginator<'a, T> {
    pub(crate) fn new(http: &'a Client, route: Route, filter: Option<Filter>) -> Self {
        Self {
            http,
            route,
            filter: filter.unwrap_or_default(),
            state: State::Start,
            phantom: PhantomData,
        }
    }

    pub async fn next(&mut self) -> Result<Option<Page<T>>, PaginateError> {
        let state = std::mem::replace(&mut self.state, State::Completed);

        let filter = self.filter.clone();

        let filter = match state {
            State::Start => filter,
            State::Next { offset, limit } => filter.offset((offset + limit) as usize),
            State::Completed => return Ok(None),
        };

        let req = RequestBuilder::from_route(&self.route)
            .filter(filter)
            .empty()
            .map_err(PaginateError::Request)?;

        let list = self
            .http
            .request::<List<T>>(req)
            .await
            .map_err(PaginateError::Request)?
            .data()
            .await
            .map_err(PaginateError::Body)?;

        if list.data.is_empty() {
            return Ok(None);
        }

        self.state = State::Next {
            offset: list.offset,
            limit: list.limit,
        };

        Ok(Some(Page(list)))
    }
}

mod private {
    use crate::request::files;
    use crate::request::games;
    use crate::request::mods;
    use crate::request::user;

    pub trait Sealed {}

    impl Sealed for games::GetGames<'_> {}
    impl Sealed for games::tags::GetGameTags<'_> {}

    impl Sealed for mods::GetMods<'_> {}
    impl Sealed for mods::comments::GetModComments<'_> {}
    impl Sealed for mods::dependencies::GetModDependencies<'_> {}
    impl Sealed for mods::events::GetModEvents<'_> {}
    impl Sealed for mods::events::GetModsEvents<'_> {}
    impl Sealed for mods::stats::GetModsStats<'_> {}
    impl Sealed for mods::tags::GetModTags<'_> {}

    impl Sealed for files::GetFiles<'_> {}
    impl Sealed for files::multipart::GetMultipartUploadParts<'_> {}
    impl Sealed for files::multipart::GetMultipartUploadSessions<'_> {}

    impl Sealed for user::GetMutedUsers<'_> {}
    impl Sealed for user::GetUserFiles<'_> {}
    impl Sealed for user::GetUserGames<'_> {}
    impl Sealed for user::GetUserMods<'_> {}
    impl Sealed for user::GetUserRatings<'_> {}
    impl Sealed for user::GetUserSubscriptions<'_> {}
}
