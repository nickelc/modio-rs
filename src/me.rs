//! Me interface
use crate::error::Result;
use crate::files::MyFiles;
use crate::games::MyGames;
use crate::mods::MyMods;
use crate::prelude::*;
use crate::types::mods::Mod;
use crate::types::User;

pub use crate::types::mods::Rating;
pub use crate::types::{Event, EventType};

/// Interface for resources owned by the authenticated user or is team member of.
pub struct Me {
    modio: Modio,
}

impl Me {
    pub(crate) fn new(modio: Modio) -> Self {
        Self { modio }
    }

    /// Return the authenticated user. [required: token]
    pub async fn authenticated_user(&self) -> Result<User> {
        token_required!(self.modio);
        self.modio.get("/me").await
    }

    /// Return a reference to an interface that provides access to games the authenticated user
    /// added or is a team member of.
    pub fn games(&self) -> MyGames {
        MyGames::new(self.modio.clone())
    }

    /// Return a reference to an interface that provides access to mods the authenticated user
    /// added or is a team member of.
    pub fn mods(&self) -> MyMods {
        MyMods::new(self.modio.clone())
    }

    /// Return a reference to an interface that provides access to modfiles the authenticated user
    /// uploaded.
    pub fn files(&self) -> MyFiles {
        MyFiles::new(self.modio.clone())
    }

    /*
    /// Provides a stream the events that have been fired specific to the authenticated user.
    /// [required: token]
    ///
    /// See [Filters and sorting](filters/events/index.html).
    pub fn events(&self, filter: &Filter) -> Stream<Event> {
        token_required!(s self.modio);
        let mut uri = vec!["/me/events".to_owned()];
        let query = filter.to_query_string();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.stream(&uri.join("?"))
    }

    /// Provides a stream over all mod's the authenticated user is subscribed to. [required: token]
    ///
    /// See [Filters and sorting](filters/subscriptions/index.html).
    pub fn subscriptions(&self, filter: &Filter) -> Stream<Mod> {
        token_required!(s self.modio);
        let mut uri = vec!["/me/subscribed".to_owned()];
        let query = filter.to_query_string();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.stream(&uri.join("?"))
    }

    /// Provides a stream over all mod rating's submitted by the authenticated user. [required:
    /// token]
    ///
    /// See [Filters and sorting](filters/ratings/index.html).
    pub fn ratings(&self, filter: &Filter) -> Stream<Rating> {
        token_required!(s self.modio);
        let mut uri = vec!["/me/ratings".to_owned()];
        let query = filter.to_query_string();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.stream(&uri.join("?"))
    }
    */
}

/// Filters for events, subscriptions and ratings.
#[rustfmt::skip]
pub mod filters {
    /// User event filters and sorting.
    ///
    /// # Filters
    /// - Id
    /// - GameId
    /// - ModId
    /// - UserId
    /// - DateAdded
    /// - EventType
    ///
    /// # Sorting
    /// - Id
    /// - DateAdded
    ///
    /// See the [modio docs](https://docs.mod.io/#get-user-events) for more information.
    ///
    /// By default this returns up to `100` items. You can limit the result by using `limit` and
    /// `offset`.
    ///
    /// # Example
    /// ```
    /// use modio::filter::prelude::*;
    /// use modio::me::filters::events::EventType as Filter;
    /// use modio::mods::EventType;
    ///
    /// let filter = Id::gt(1024).and(Filter::eq(EventType::ModfileChanged));
    /// ```
    pub mod events {
        #[doc(inline)]
        pub use crate::filter::prelude::Id;
        #[doc(inline)]
        pub use crate::filter::prelude::ModId;
        #[doc(inline)]
        pub use crate::filter::prelude::DateAdded;

        #[doc(inline)]
        pub use crate::mods::filters::events::UserId;
        #[doc(inline)]
        pub use crate::mods::filters::events::EventType;

        filter!(GameId, GAME_ID, "game_id", Eq, NotEq, In, Cmp, OrderBy);
    }

    /// Subscriptions filters and sorting.
    ///
    /// # Filters
    /// - Fulltext
    /// - Id
    /// - GameId
    /// - Status
    /// - Visible
    /// - SubmittedBy
    /// - DateAdded
    /// - DateUpdated
    /// - DateLive
    /// - MaturityOption
    /// - Name
    /// - NameId
    /// - Summary
    /// - Description
    /// - Homepage
    /// - Modfile
    /// - MetadataBlob
    /// - MetadataKVP
    /// - Tags
    ///
    /// # Sorting
    /// - Id
    /// - Name
    /// - Downloads
    /// - Popular
    /// - Ratings
    /// - Subscribers
    ///
    /// See the [mod.io docs](https://docs.mod.io/#get-user-subscriptions) for more information.
    ///
    /// By default this returns up to `100` items. you can limit the result by using `limit` and
    /// `offset`.
    ///
    /// # Example
    /// ```
    /// use modio::filter::prelude::*;
    /// use modio::me::filters::subscriptions::Id;
    ///
    /// let filter = Id::_in(vec![1, 2]).order_by(Id::desc());
    /// ```
    pub mod subscriptions {
        #[doc(inline)]
        pub use crate::filter::prelude::Fulltext;
        #[doc(inline)]
        pub use crate::filter::prelude::Id;
        #[doc(inline)]
        pub use crate::filter::prelude::Name;
        #[doc(inline)]
        pub use crate::filter::prelude::NameId;

        #[doc(inline)]
        pub use crate::mods::filters::GameId;
        #[doc(inline)]
        pub use crate::mods::filters::Status;
        #[doc(inline)]
        pub use crate::mods::filters::Visible;
        #[doc(inline)]
        pub use crate::mods::filters::SubmittedBy;
        #[doc(inline)]
        pub use crate::mods::filters::DateAdded;
        #[doc(inline)]
        pub use crate::mods::filters::DateUpdated;
        #[doc(inline)]
        pub use crate::mods::filters::DateLive;
        #[doc(inline)]
        pub use crate::mods::filters::MaturityOption;
        #[doc(inline)]
        pub use crate::mods::filters::Summary;
        #[doc(inline)]
        pub use crate::mods::filters::Description;
        #[doc(inline)]
        pub use crate::mods::filters::Homepage;
        #[doc(inline)]
        pub use crate::mods::filters::Modfile;
        #[doc(inline)]
        pub use crate::mods::filters::MetadataBlob;
        #[doc(inline)]
        pub use crate::mods::filters::MetadataKVP;
        #[doc(inline)]
        pub use crate::mods::filters::Tags;

        #[doc(inline)]
        pub use crate::mods::filters::Downloads;
        #[doc(inline)]
        pub use crate::mods::filters::Popular;
        #[doc(inline)]
        pub use crate::mods::filters::Ratings;
        #[doc(inline)]
        pub use crate::mods::filters::Subscribers;
    }

    /// Rating filters and sorting.
    ///
    /// # Filters
    /// - GameId
    /// - ModId
    /// - Rating
    /// - DateAdded
    ///
    /// # Sorting
    /// - GameId
    /// - ModId
    /// - Rating
    /// - DateAdded
    ///
    /// See the [mod.io docs](https://docs.mod.io/#get-user-ratings) for more information.
    ///
    /// By default this returns up to `100` items. You can limit the result by using `limit` and
    /// `offset`.
    ///
    /// # Example
    /// ```
    /// use modio::filter::prelude::*;
    /// use modio::me::filters::ratings::GameId;
    /// use modio::me::filters::ratings::DateAdded;
    /// use modio::me::filters::ratings::Rating;
    ///
    /// let filter = GameId::_in(vec![1, 2]).order_by(DateAdded::desc());
    ///
    /// let filter = Rating::positive().order_by(DateAdded::desc());
    /// ```
    pub mod ratings {
        use crate::filter::prelude::*;

        #[doc(inline)]
        pub use crate::filter::prelude::ModId;

        filter!(GameId, GAME_ID, "game_id", Eq, NotEq, In, Cmp, OrderBy);
        filter!(Rating, RATING, "rating", Eq, NotEq, In, Cmp, OrderBy);
        filter!(DateAdded, DATE_ADDED, "date_added", Eq, NotEq, In, Cmp, OrderBy);

        impl Rating {
            pub fn positive() -> Filter {
                Rating::eq(1)
            }

            pub fn negative() -> Filter {
                Rating::eq(-1)
            }
        }
    }
}
