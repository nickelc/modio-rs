//! Mod comments interface
use crate::prelude::*;
pub use crate::types::mods::Comment;

/// Interface for comments of a mod.
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

    /// Returns a `Query` interface to retrieve all comments.
    ///
    /// See [Filters and sorting](filters).
    pub fn search(&self, filter: Filter) -> Query<Comment> {
        let route = Route::GetModComments {
            game_id: self.game,
            mod_id: self.mod_id,
        };
        Query::new(self.modio.clone(), route, filter)
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

    /// Add a new comment. [required: token]
    pub async fn add<S>(self, content: S, reply_id: Option<u32>) -> Result<Comment>
    where
        S: Into<String>,
    {
        let route = Route::AddModComment {
            game_id: self.game,
            mod_id: self.mod_id,
        };
        let content = content.into();
        let data = CommentOptions { content, reply_id };
        self.modio.request(route).form(&data).send().await
    }

    /// Edit a comment by id. [required: token]
    pub async fn edit<S>(self, id: u32, content: S) -> Result<Comment>
    where
        S: Into<String>,
    {
        let route = Route::EditModComment {
            game_id: self.game,
            mod_id: self.mod_id,
            comment_id: id,
        };
        let data = CommentOptions {
            content: content.into(),
            reply_id: None,
        };
        self.modio.request(route).form(&data).send().await
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
/// See [modio docs](https://docs.mod.io/#get-mod-comments) for more information.
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

use serde::Serialize;

#[derive(Serialize)]
struct CommentOptions {
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_id: Option<u32>,
}
