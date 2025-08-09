mod get_mod_stats;
mod get_mods_stats;

pub use get_mod_stats::GetModStats;
pub use get_mods_stats::GetModsStats;

/// Mod statistics filters & sorting
///
/// # Filters
/// - `ModId`
/// - `Popularity`
/// - `Downloads`
/// - `Subscribers`
/// - `RatingsPositive`
/// - `RatingsNegative`
///
/// # Sorting
/// - `ModId`
/// - `Popularity`
/// - `Downloads`
/// - `Subscribers`
/// - `RatingsPositive`
/// - `RatingsNegative`
///
/// # Example
/// ```
/// use modio::request::filter::prelude::*;
/// use modio::request::mods::stats::filters::{ModId, Popularity};
///
/// let filter = ModId::_in(vec![1, 2]).order_by(Popularity::desc());
/// ```
#[rustfmt::skip]
pub mod filters {
    #[doc(inline)]
    pub use crate::request::filter::prelude::ModId;

    filter!(Popularity, POPULARITY, "popularity_rank_position", Eq, NotEq, In, Cmp, OrderBy);
    filter!(Downloads, DOWNLOADS, "downloads_total", Eq, NotEq, In, Cmp, OrderBy);
    filter!(Subscribers, SUBSCRIBERS, "subscribers_total", Eq, NotEq, In, Cmp, OrderBy);
    filter!(RatingsPositive, RATINGS_POSITIVE, "ratings_positive", Eq, NotEq, In, Cmp, OrderBy);
    filter!(RatingsNegative, RATINGS_NEGATIVE, "ratings_negative", Eq, NotEq, In, Cmp, OrderBy);
}
