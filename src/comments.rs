//! Mod comments interface

use crate::prelude::*;
pub use crate::types::mods::Comment;

pub struct Comments<C>
where
    C: Clone + Connect + 'static,
{
    modio: Modio<C>,
    game: u32,
    mod_id: u32,
}

impl<C: Clone + Connect + 'static> Comments<C> {
    pub fn new(modio: Modio<C>, game: u32, mod_id: u32) -> Self {
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
    pub fn list(&self, options: &CommentsListOptions) -> Future<List<Comment>> {
        let mut uri = vec![self.path("")];
        let query = options.to_query_params();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.get(&uri.join("?"))
    }

    /// Provides a stream over all comments of the mod.
    pub fn iter(&self, options: &CommentsListOptions) -> Stream<Comment> {
        let mut uri = vec![self.path("")];
        let query = options.to_query_params();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.stream(&uri.join("?"))
    }

    /// Return comment by id.
    pub fn get(&self, id: u32) -> Future<Comment> {
        self.modio.get(&self.path(&format!("/{}", id)))
    }

    /// Delete a comment by id.
    pub fn delete(&self, id: u32) -> Future<()> {
        self.modio
            .delete(&self.path(&format!("/{}", id)), Body::empty())
    }
}

filter_options! {
    /// Options used to filter comment listings
    ///
    /// # Filter parameters
    /// - _q
    /// - id
    /// - mod_id
    /// - submitted_by
    /// - date_added
    /// - reply_id
    /// - thread_position
    /// - karma
    /// - content
    ///
    /// # Sorting
    /// - id
    /// - mod_id
    /// - submitted_by
    /// - date_added
    ///
    /// See [modio docs](https://docs.mod.io/#get-all-mod-comments) for more information.
    ///
    /// By default this returns up to `100` items. You can limit the result using `limit` and
    /// `offset`.
    /// # Example
    /// ```
    /// use modio::filter::{Order, Operator};
    /// use modio::comments::CommentsListOptions;
    ///
    /// let mut opts = CommentsListOptions::new();
    /// opts.id(Operator::In, vec![1, 2]);
    /// opts.sort_by(CommentsListOptions::ID, Order::Desc);
    /// ```
    #[derive(Debug)]
    pub struct CommentsListOptions {
        Filters
        - id = "id";
        - mod_id = "mod_id";
        - submitted_by = "submitted_by";
        - date_added = "date_added";
        - reply_id = "reply_id";
        - thread_position = "thread_position";
        - karma = "karma";
        - content = "content";

        Sort
        - ID = "id";
        - MOD_ID = "mod_id";
        - SUBMITTED_BY = "submitted_by";
        - DATE_ADDED = "date_added";
    }
}
