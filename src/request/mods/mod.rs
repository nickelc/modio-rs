mod add_mod;
mod delete_mod;
mod edit_mod;
mod get_mod;
mod get_mod_team_members;
mod get_mods;
mod submit_mod_rating;

pub mod comments;
pub mod dependencies;
pub mod events;
pub mod media;
pub mod metadata;
pub mod stats;
pub mod subscribe;
pub mod tags;

pub use add_mod::AddMod;
pub use delete_mod::DeleteMod;
pub use edit_mod::EditMod;
pub use get_mod::GetMod;
pub use get_mod_team_members::GetModTeamMembers;
pub use get_mods::GetMods;
pub use submit_mod_rating::SubmitModRating;

/// Mod filters & sorting
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
/// See the [modio docs](https://docs.mod.io/restapiref/#get-mods) for more information.
///
/// By default this returns up to `100` items. you can limit the result by using `limit` and
/// `offset`.
///
/// # Example
/// ```
/// use modio::request::filter::prelude::*;
/// use modio::request::mods::filters::Id;
/// use modio::request::mods::filters::GameId;
/// use modio::request::mods::filters::Tags;
///
/// let filter = Id::_in(vec![1, 2]).order_by(Id::desc());
///
/// let filter = GameId::eq(6).and(Tags::eq("foobar")).limit(10);
/// ```
#[rustfmt::skip]
pub mod filters {
    #[doc(inline)]
    pub use crate::request::filter::prelude::Fulltext;
    #[doc(inline)]
    pub use crate::request::filter::prelude::Id;
    #[doc(inline)]
    pub use crate::request::filter::prelude::Name;
    #[doc(inline)]
    pub use crate::request::filter::prelude::NameId;
    #[doc(inline)]
    pub use crate::request::filter::prelude::Status;
    #[doc(inline)]
    pub use crate::request::filter::prelude::DateAdded;
    #[doc(inline)]
    pub use crate::request::filter::prelude::DateUpdated;
    #[doc(inline)]
    pub use crate::request::filter::prelude::DateLive;
    #[doc(inline)]
    pub use crate::request::filter::prelude::SubmittedBy;

    filter!(GameId, GAME_ID, "game_id", Eq, NotEq, In, Cmp, OrderBy);
    filter!(Visible, VISIBLE, "visible", Eq);
    filter!(MaturityOption, MATURITY_OPTION, "maturity_option", Eq, Cmp, Bit);
    filter!(Summary, SUMMARY, "summary", Like);
    filter!(Description, DESCRIPTION, "description", Like);
    filter!(Homepage, HOMEPAGE, "homepage_url", Eq, NotEq, Like, In);
    filter!(Modfile, MODFILE, "modfile", Eq, NotEq, In, Cmp);
    filter!(MetadataBlob, METADATA_BLOB, "metadata_blob", Eq, NotEq, Like);
    filter!(MetadataKVP, METADATA_KVP, "metadata_kvp", Eq, NotEq, Like);
    filter!(Tags, TAGS, "tags", Eq, NotEq, Like, In);

    filter!(Downloads, DOWNLOADS, "downloads", OrderBy);
    filter!(Popular, POPULAR, "popular", OrderBy);
    filter!(Ratings, RATINGS, "ratings", OrderBy);
    filter!(Subscribers, SUBSCRIBERS, "subscribers", OrderBy);
}
