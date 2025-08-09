mod get_mod_events;
mod get_mods_events;

pub use get_mod_events::GetModEvents;
pub use get_mods_events::GetModsEvents;

/// Mod event filters and sorting.
///
/// # Filters
/// - `Id`
/// - `ModId`
/// - `UserId`
/// - `DateAdded`
/// - `EventType`
///
/// # Sorting
/// - `Id`
/// - `DateAdded`
///
/// See the [modio docs](https://docs.mod.io/restapiref/#events) for more information.
///
/// By default this returns up to `100` items. You can limit the result by using `limit` and
/// `offset`.
///
/// # Example
/// ```
/// use modio::request::filter::prelude::*;
/// use modio::request::mods::events::filters::EventType as Filter;
/// use modio::types::mods::EventType;
///
/// let filter = Id::gt(1024).and(Filter::eq(EventType::MODFILE_CHANGED));
/// ```
pub mod filters {
    #[doc(inline)]
    pub use crate::request::filter::prelude::DateAdded;
    #[doc(inline)]
    pub use crate::request::filter::prelude::Id;
    #[doc(inline)]
    pub use crate::request::filter::prelude::ModId;

    filter!(UserId, USER_ID, "user_id", Eq, NotEq, In, Cmp, OrderBy);
    filter!(EventType, EVENT_TYPE, "event_type", Eq, NotEq, In, OrderBy);
}
