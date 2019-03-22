//! Team members interface
use url::form_urlencoded;

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
    pub fn list(&self, filter: &Filter) -> Future<List<TeamMember>> {
        let mut uri = vec![self.path("")];
        let query = filter.to_query_params();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.get(&uri.join("?"))
    }

    /// Provids a stream over all team members.
    ///
    /// See [Filters and sorting](filters/index.html).
    pub fn iter(&self, filter: &Filter) -> Stream<TeamMember> {
        let mut uri = vec![self.path("")];
        let query = filter.to_query_params();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.stream(&uri.join("?"))
    }

    /// Add a team member by email. [required: token]
    pub fn add(&self, options: &InviteTeamMemberOptions) -> Future<String> {
        token_required!(self.modio);
        let params = options.to_query_params();
        Box::new(
            self.modio
                .post::<ModioMessage, _>(&self.path(""), params)
                .map(|m| m.message),
        )
    }

    /// Edit a team member by id. [required: token]
    pub fn edit(&self, id: u32, options: &EditTeamMemberOptions) -> Future<String> {
        token_required!(self.modio);
        let params = options.to_query_params();
        Box::new(
            self.modio
                .put::<ModioMessage, _>(&self.path(&format!("/{}", id)), params)
                .map(|m| m.message),
        )
    }

    /// Delete a team member by id. [required: token]
    pub fn delete(&self, id: u32) -> Future<()> {
        token_required!(self.modio);
        self.modio
            .delete(&self.path(&format!("/{}", id)), RequestBody::Empty)
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
    email: String,
    level: TeamLevel,
    position: Option<String>,
}

impl InviteTeamMemberOptions {
    pub fn builder<T>(email: T, level: TeamLevel) -> InviteTeamMemberOptionsBuilder
    where
        T: Into<String>,
    {
        InviteTeamMemberOptionsBuilder::new(email, level)
    }
}

impl QueryParams for InviteTeamMemberOptions {
    fn to_query_params(&self) -> String {
        form_urlencoded::Serializer::new(String::new())
            .append_pair("email", &self.email)
            .append_pair("level", &self.level.value().to_string())
            .extend_pairs(self.position.iter().map(|p| ("position", p)))
            .finish()
    }
}

pub struct InviteTeamMemberOptionsBuilder(InviteTeamMemberOptions);

impl InviteTeamMemberOptionsBuilder {
    pub fn new<T: Into<String>>(email: T, level: TeamLevel) -> Self {
        InviteTeamMemberOptionsBuilder(InviteTeamMemberOptions {
            email: email.into(),
            level,
            position: None,
        })
    }

    pub fn position<T: Into<String>>(&mut self, position: T) -> &mut Self {
        self.0.position = Some(position.into());
        self
    }

    pub fn build(&self) -> InviteTeamMemberOptions {
        InviteTeamMemberOptions {
            email: self.0.email.clone(),
            level: self.0.level,
            position: self.0.position.clone(),
        }
    }
}

#[derive(Debug, Default)]
pub struct EditTeamMemberOptions {
    level: Option<TeamLevel>,
    position: Option<String>,
}

impl EditTeamMemberOptions {
    pub fn builder() -> EditTeamMemberOptionsBuilder {
        EditTeamMemberOptionsBuilder::new()
    }
}

impl QueryParams for EditTeamMemberOptions {
    fn to_query_params(&self) -> String {
        form_urlencoded::Serializer::new(String::new())
            .extend_pairs(self.level.iter().map(|l| ("level", l.value().to_string())))
            .extend_pairs(self.position.iter().map(|p| ("position", p)))
            .finish()
    }
}

#[derive(Default)]
pub struct EditTeamMemberOptionsBuilder(EditTeamMemberOptions);

impl EditTeamMemberOptionsBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn level(&mut self, level: TeamLevel) -> &mut Self {
        self.0.level = Some(level);
        self
    }

    pub fn position<T: Into<String>>(&mut self, position: T) -> &mut Self {
        self.0.position = Some(position.into());
        self
    }

    pub fn build(&self) -> EditTeamMemberOptions {
        EditTeamMemberOptions {
            level: self.0.level,
            position: self.0.position.clone(),
        }
    }
}
