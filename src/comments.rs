//! Mod comments interface
use crate::error::Result;
use crate::prelude::*;
pub use crate::types::mods::Comment;

pub struct Comments {
    modio: Modio,
    game: u32,
    mod_id: u32,
}

impl Comments {
    pub fn new(modio: Modio, game: u32, mod_id: u32) -> Self {
        Self {
            modio,
            game,
            mod_id,
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/games/{}/mods/{}/comments{}", self.game, self.mod_id, more)
    }

    /// List all comments.
    ///
    /// See [Filters and sorting](filters/index.html).
    pub async fn list(&self, filter: &Filter) -> Result<List<Comment>> {
        let mut uri = vec![self.path("")];
        let query = filter.to_query_string();
        if !query.is_empty() {
            uri.push(query);
        }
        let url = uri.join("?");
        self.modio.get(&url).await
    }

    /*
    /// Provides a stream over all comments of the mod.
    ///
    /// See [Filters and sorting](filters/index.html).
    pub fn iter(&self, filter: &Filter) -> Stream<Comment> {
        let mut uri = vec![self.path("")];
        let query = filter.to_query_string();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.stream(&uri.join("?"))
    }
    */

    /// Return comment by id.
    pub async fn get(&self, id: u32) -> Result<Comment> {
        let url = self.path(&format!("/{}", id));
        self.modio.get(&url).await
    }

    /// Delete a comment by id. [required: token]
    pub async fn delete(&self, id: u32) -> Result<()> {
        token_required!(self.modio);
        let url = self.path(&format!("/{}", id));
        self.modio.delete(&url, RequestBody::Empty).await
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
