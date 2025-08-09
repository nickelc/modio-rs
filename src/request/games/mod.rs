mod add_game_media;
mod get_game;
mod get_game_stats;
mod get_games;

pub mod tags;

pub use add_game_media::AddGameMedia;
pub use get_game::GetGame;
pub use get_game_stats::GetGameStats;
pub use get_games::GetGames;

/// Game filters and sorting.
///
/// # Filters
/// - `Fulltext`
/// - `Id`
/// - `Status`
/// - `SubmittedBy`
/// - `DateAdded`
/// - `DateUpdated`
/// - `DateLive`
/// - `Name`
/// - `NameId`
/// - `Summary`
/// - `InstructionsUrl`
/// - `UgcName`
/// - `PresentationOption`
/// - `SubmissionOption`
/// - `CurationOption`
/// - `CommunityOptions`
/// - `RevenueOptions`
/// - `ApiAccessOptions`
/// - `MaturityOptions`
///
/// # Sorting
/// - `Id`
/// - `Status`
/// - `Name`
/// - `NameId`
/// - `DateUpdated`
///
/// See [modio docs](https://docs.mod.io/restapiref/#get-games) for more information.
///
/// By default this returns up to `100` items. You can limit the result by using `limit` and
/// `offset`.
///
/// # Example
/// ```
/// use modio::request::filter::prelude::*;
/// use modio::request::games::filters::Id;
///
/// let filter = Id::_in(vec![1, 2]).order_by(Id::desc());
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

    filter!(Summary, SUMMARY, "summary", Eq, NotEq, Like);
    filter!(InstructionsUrl, INSTRUCTIONS_URL, "instructions_url", Eq, NotEq, In, Like);
    filter!(UgcName, UGC_NAME, "ugc_name", Eq, NotEq, In, Like);
    filter!(PresentationOption, PRESENTATION_OPTION, "presentation_option", Eq, NotEq, In, Cmp, Bit);
    filter!(SubmissionOption, SUBMISSION_OPTION, "submission_option", Eq, NotEq, In, Cmp, Bit);
    filter!(CurationOption, CURATION_OPTION, "curation_option", Eq, NotEq, In, Cmp, Bit);
    filter!(CommunityOptions, COMMUNITY_OPTIONS, "community_options", Eq, NotEq, In, Cmp, Bit);
    filter!(RevenueOptions, REVENUE_OPTIONS, "revenue_options", Eq, NotEq, In, Cmp, Bit);
    filter!(ApiAccessOptions, API_ACCESS_OPTIONS, "api_access_options", Eq, NotEq, In, Cmp, Bit);
    filter!(MaturityOptions, MATURITY_OPTIONS, "maturity_options", Eq, NotEq, In, Cmp, Bit);
}
