mod get_authenticated_user;
mod get_muted_users;
mod get_user_events;
mod get_user_files;
mod get_user_games;
mod get_user_mods;
mod get_user_ratings;
mod get_user_subscriptions;
mod mute_user;
mod unmute_user;

pub use get_authenticated_user::GetAuthenticatedUser;
pub use get_muted_users::GetMutedUsers;
pub use get_user_events::GetUserEvents;
pub use get_user_files::GetUserFiles;
pub use get_user_games::GetUserGames;
pub use get_user_mods::GetUserMods;
pub use get_user_ratings::GetUserRatings;
pub use get_user_subscriptions::GetUserSubscriptions;
pub use mute_user::MuteUser;
pub use unmute_user::UnmuteUser;

/// Filters for events, subscriptions and ratings.
#[rustfmt::skip]
pub mod filters {
    #[doc(inline)]
    pub use crate::request::games::filters as games;
    #[doc(inline)]
    pub use crate::request::mods::filters as mods;
    #[doc(inline)]
    pub use crate::request::files::filters as files;

    /// User event filters and sorting.
    ///
    /// # Filters
    /// - `Id`
    /// - `GameId`
    /// - `ModId`
    /// - `UserId`
    /// - `DateAdded`
    /// - `EventType`
    ///
    /// # Sorting
    /// - `Id`
    /// - `DateAdded`
    ///
    /// See the [modio docs](https://docs.mod.io/restapiref/#get-user-events) for more information.
    ///
    /// By default this returns up to `100` items. You can limit the result by using `limit` and
    /// `offset`.
    ///
    /// # Example
    /// ```
    /// use modio::request::filter::prelude::*;
    /// use modio::request::user::filters::events::EventType as Filter;
    /// use modio::types::mods::EventType;
    ///
    /// let filter = Id::gt(1024).and(Filter::eq(EventType::MODFILE_CHANGED));
    /// ```
    pub mod events {
        #[doc(inline)]
        pub use crate::request::filter::prelude::Id;
        #[doc(inline)]
        pub use crate::request::filter::prelude::ModId;
        #[doc(inline)]
        pub use crate::request::filter::prelude::DateAdded;

        #[doc(inline)]
        pub use crate::request::mods::events::filters::UserId;
        #[doc(inline)]
        pub use crate::request::mods::events::filters::EventType;

        filter!(GameId, GAME_ID, "game_id", Eq, NotEq, In, Cmp, OrderBy);
    }

    /// Subscriptions filters and sorting.
    ///
    /// # Filters
    /// - `Fulltext`
    /// - `Id`
    /// - `GameId`
    /// - `Status`
    /// - `Visible`
    /// - `SubmittedBy`
    /// - `DateAdded`
    /// - `DateUpdated`
    /// - `DateLive`
    /// - `MaturityOption`
    /// - `Name`
    /// - `NameId`
    /// - `Summary`
    /// - `Description`
    /// - `Homepage`
    /// - `Modfile`
    /// - `MetadataBlob`
    /// - `MetadataKVP`
    /// - `Tags`
    ///
    /// # Sorting
    /// - `Id`
    /// - `Name`
    /// - `Downloads`
    /// - `Popular`
    /// - `Ratings`
    /// - `Subscribers`
    ///
    /// See the [mod.io docs](https://docs.mod.io/restapiref/#get-user-subscriptions) for more information.
    ///
    /// By default this returns up to `100` items. you can limit the result by using `limit` and
    /// `offset`.
    ///
    /// # Example
    /// ```
    /// use modio::request::filter::prelude::*;
    /// use modio::request::user::filters::subscriptions::Id;
    ///
    /// let filter = Id::_in(vec![1, 2]).order_by(Id::desc());
    /// ```
    pub mod subscriptions {
        #[doc(inline)]
        pub use crate::request::filter::prelude::Fulltext;
        #[doc(inline)]
        pub use crate::request::filter::prelude::Id;
        #[doc(inline)]
        pub use crate::request::filter::prelude::Name;
        #[doc(inline)]
        pub use crate::request::filter::prelude::NameId;

        #[doc(inline)]
        pub use crate::request::mods::filters::GameId;
        #[doc(inline)]
        pub use crate::request::mods::filters::Status;
        #[doc(inline)]
        pub use crate::request::mods::filters::Visible;
        #[doc(inline)]
        pub use crate::request::mods::filters::SubmittedBy;
        #[doc(inline)]
        pub use crate::request::mods::filters::DateAdded;
        #[doc(inline)]
        pub use crate::request::mods::filters::DateUpdated;
        #[doc(inline)]
        pub use crate::request::mods::filters::DateLive;
        #[doc(inline)]
        pub use crate::request::mods::filters::MaturityOption;
        #[doc(inline)]
        pub use crate::request::mods::filters::Summary;
        #[doc(inline)]
        pub use crate::request::mods::filters::Description;
        #[doc(inline)]
        pub use crate::request::mods::filters::Homepage;
        #[doc(inline)]
        pub use crate::request::mods::filters::Modfile;
        #[doc(inline)]
        pub use crate::request::mods::filters::MetadataBlob;
        #[doc(inline)]
        pub use crate::request::mods::filters::MetadataKVP;
        #[doc(inline)]
        pub use crate::request::mods::filters::Tags;

        #[doc(inline)]
        pub use crate::request::mods::filters::Downloads;
        #[doc(inline)]
        pub use crate::request::mods::filters::Popular;
        #[doc(inline)]
        pub use crate::request::mods::filters::Ratings;
        #[doc(inline)]
        pub use crate::request::mods::filters::Subscribers;
    }

    /// Rating filters and sorting.
    ///
    /// # Filters
    /// - `GameId`
    /// - `ModId`
    /// - `Rating`
    /// - `DateAdded`
    ///
    /// # Sorting
    /// - `GameId`
    /// - `ModId`
    /// - `Rating`
    /// - `DateAdded`
    ///
    /// See the [mod.io docs](https://docs.mod.io/restapiref/#get-user-ratings) for more information.
    ///
    /// By default this returns up to `100` items. You can limit the result by using `limit` and
    /// `offset`.
    ///
    /// # Example
    /// ```
    /// use modio::request::filter::prelude::*;
    /// use modio::request::user::filters::ratings::GameId;
    /// use modio::request::user::filters::ratings::DateAdded;
    /// use modio::request::user::filters::ratings::Rating;
    ///
    /// let filter = GameId::_in(vec![1, 2]).order_by(DateAdded::desc());
    ///
    /// let filter = Rating::positive().order_by(DateAdded::desc());
    /// ```
    pub mod ratings {
        use crate::request::filter::prelude::*;

        #[doc(inline)]
        pub use crate::request::filter::prelude::ModId;

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
