//! Mod comments interface
use crate::prelude::*;
pub use crate::types::mods::Comment;

pub struct Comments {
    modio: Modio,
    game: u32,
    mod_id: u32,
}

impl Comments {
    pub(crate) fn new(modio: Modio, game: u32, mod_id: u32) -> Self {
        Self {
            modio,
            game,
            mod_id,
        }
    }

    /// List all comments.
    ///
    /// See [Filters and sorting](filters/index.html).
    pub async fn list(self, filter: Filter) -> Result<List<Comment>> {
        let route = Route::GetModComments {
            game_id: self.game,
            mod_id: self.mod_id,
        };
        self.modio
            .request(route)
            .query(filter.to_query_string())
            .send()
            .await
    }

    /// Provides a stream over all comments of the mod.
    ///
    /// See [Filters and sorting](filters/index.html).
    pub fn iter(self, filter: Filter) -> impl Stream<Item = Result<Comment>> {
        let route = Route::GetModComments {
            game_id: self.game,
            mod_id: self.mod_id,
        };
        self.modio.stream(route, filter)
    }

    /// Return comment by id.
    pub async fn get(self, id: u32) -> Result<Comment> {
        let route = Route::GetModComment {
            game_id: self.game,
            mod_id: self.mod_id,
            comment_id: id,
        };
        self.modio.request(route).send().await
    }

    /// Delete a comment by id. [required: token]
    pub async fn delete(self, id: u32) -> Result<()> {
        let route = Route::DeleteModComment {
            game_id: self.game,
            mod_id: self.mod_id,
            comment_id: id,
        };
        self.modio.request(route).send().await
    }
}

/// Comment filters and sorting.
///
/// # Filters
/// - Fulltext
/// - Id
/// - ModId
/// - SubmittedBy
/// - DateAdded
/// - ReplyId
/// - ThreadPosition
/// - Karma
/// - Content
///
/// # Sorting
/// - Id
/// - ModId
/// - SubmittedBy
/// - DateAdded
///
/// See [modio docs](https://docs.mod.io/#get-all-mod-comments) for more information.
///
/// By default this returns up to `100` items. You can limit the result by using `limit` and
/// `offset`.
///
/// # Example
/// ```
/// use modio::filter::prelude::*;
/// use modio::comments::filters::Id;
///
/// let filter = Id::_in(vec![1, 2]).order_by(Id::desc());
/// ```
#[rustfmt::skip]
pub mod filters {
    #[doc(inline)]
    pub use crate::filter::prelude::Fulltext;
    #[doc(inline)]
    pub use crate::filter::prelude::Id;
    #[doc(inline)]
    pub use crate::filter::prelude::ModId;
    #[doc(inline)]
    pub use crate::filter::prelude::DateAdded;
    #[doc(inline)]
    pub use crate::filter::prelude::SubmittedBy;

    filter!(ReplyId, REPLY_ID, "reply_id", Eq, NotEq, In, Cmp);
    filter!(ThreadPosition, THREAD_POSITION, "thread_position", Eq, NotEq, In, Like);
    filter!(Karma, KARMA, "karma", Eq, NotEq, In, Cmp);
    filter!(Content, CONTENT, "content", Eq, NotEq, Like);
}
