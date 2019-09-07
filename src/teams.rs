//! Team members interface
use url::form_urlencoded;

use crate::error::Result;
use crate::prelude::*;

pub use crate::types::mods::{TeamLevel, TeamMember};

/// Interface for the team members of a mod.
pub struct Members {
    modio: Modio,
    game: u32,
    mod_id: u32,
}

impl Members {
    pub(crate) fn new(modio: Modio, game: u32, mod_id: u32) -> Self {
        Self {
            modio,
            game,
            mod_id,
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/games/{}/mods/{}/team{}", self.game, self.mod_id, more)
    }

    /// List all team members.
    ///
    /// See [Filters and sorting](filters/index.html).
    pub async fn list(&self, filter: &Filter) -> Result<List<TeamMember>> {
        let mut uri = vec![self.path("")];
        let query = filter.to_query_string();
        if !query.is_empty() {
            uri.push(query);
        }
        let url = uri.join("?");
        self.modio.get(&url).await
    }

    /*
    /// Provids a stream over all team members.
    ///
    /// See [Filters and sorting](filters/index.html).
    pub fn iter(&self, filter: &Filter) -> Stream<TeamMember> {
        let mut uri = vec![self.path("")];
        let query = filter.to_query_string();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.stream(&uri.join("?"))
    }
    */

    /// Add a team member by email. [required: token]
    pub async fn add(&self, options: &InviteTeamMemberOptions) -> Result<()> {
        token_required!(self.modio);
        let params = options.to_query_string();
        let url = self.path("");
        self.modio.post::<ModioMessage, _>(&url, params).await?;

        Ok(())
    }

    /// Edit a team member by id. [required: token]
    pub async fn edit(&self, id: u32, options: &EditTeamMemberOptions) -> Result<()> {
        token_required!(self.modio);
        let params = options.to_query_string();
        let url = self.path(&format!("/{}", id));
        self.modio.put::<ModioMessage, _>(&url, params).await?;

        Ok(())
    }

    /// Delete a team member by id. [required: token]
    pub async fn delete(&self, id: u32) -> Result<()> {
        token_required!(self.modio);
        let url = self.path(&format!("/{}", id));
        self.modio.delete(&url, RequestBody::Empty).await
    }
}

/// Team member filters and sorting.
///
/// # Filters
/// - Fulltext
/// - Id
/// - UserId
/// - Username
/// - Level
/// - DateAdded
/// - Position
///
/// # Sorting
/// - Id
/// - UserId
/// - Username
///
/// See [modio docs](https://docs.mod.io/#get-all-mod-team-members) for more information.
///
/// By default this returns up to `100` items. You can limit the result by using `limit` and
/// `offset`.
///
/// # Example
/// ```
/// use modio::filter::prelude::*;
/// use modio::teams::filters::Id;
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
    pub use crate::filter::prelude::DateAdded;

    filter!(UserId, USER_ID, "user_id", Eq, NotEq, In, Cmp, OrderBy);
    filter!(Username, USERNAME, "username", Eq, NotEq, In, Like, OrderBy);
    filter!(Level, LEVEL, "level", Eq, NotEq, In, Cmp, OrderBy);
    filter!(Position, POSITION, "position", Eq, NotEq, In, Like, OrderBy);
}

#[derive(Debug)]
pub struct InviteTeamMemberOptions {
    params: std::collections::BTreeMap<&'static str, String>,
}

impl InviteTeamMemberOptions {
    pub fn new<T>(email: T, level: TeamLevel) -> InviteTeamMemberOptions
    where
        T: Into<String>,
    {
        let mut params = std::collections::BTreeMap::new();
        params.insert("email", email.into());
        params.insert("level", level.to_string());
        InviteTeamMemberOptions { params }
    }

    option!(position >> "position");
}

impl QueryString for InviteTeamMemberOptions {
    fn to_query_string(&self) -> String {
        form_urlencoded::Serializer::new(String::new())
            .extend_pairs(&self.params)
            .finish()
    }
}

#[derive(Debug, Default)]
pub struct EditTeamMemberOptions {
    params: std::collections::BTreeMap<&'static str, String>,
}

impl EditTeamMemberOptions {
    option!(level: TeamLevel >> "level");
    option!(position >> "position");
}

impl QueryString for EditTeamMemberOptions {
    fn to_query_string(&self) -> String {
        form_urlencoded::Serializer::new(String::new())
            .extend_pairs(&self.params)
            .finish()
    }
}
