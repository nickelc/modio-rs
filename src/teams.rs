//! Team members interface

use url::form_urlencoded;

use crate::prelude::*;

pub use crate::types::mods::{TeamLevel, TeamMember};

/// Interface for the team members of a mod.
pub struct Members<C>
where
    C: Clone + Connect + 'static,
{
    modio: Modio<C>,
    game: u32,
    mod_id: u32,
}

impl<C: Clone + Connect + 'static> Members<C> {
    pub(crate) fn new(modio: Modio<C>, game: u32, mod_id: u32) -> Self {
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
    pub fn list(&self, options: &TeamMemberListOptions) -> Future<List<TeamMember>> {
        let mut uri = vec![self.path("")];
        let query = options.to_query_params();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.get(&uri.join("?"))
    }

    /// Provids a stream over all team members.
    pub fn iter(&self, options: &TeamMemberListOptions) -> Stream<TeamMember> {
        let mut uri = vec![self.path("")];
        let query = options.to_query_params();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.stream(&uri.join("?"))
    }

    /// Add a team member by email.
    pub fn add(&self, options: &InviteTeamMemberOptions) -> Future<ModioMessage> {
        let params = options.to_query_params();
        self.modio.post(&self.path(""), params)
    }

    /// Edit a team member by id.
    pub fn edit(&self, id: u32, options: &EditTeamMemberOptions) -> Future<ModioMessage> {
        let params = options.to_query_params();
        self.modio.put(&self.path(&format!("/{}", id)), params)
    }

    /// Delete a team member by id.
    pub fn delete(&self, id: u32) -> Future<()> {
        self.modio
            .delete(&self.path(&format!("/{}", id)), Body::empty())
    }
}

filter_options! {
    /// Options used to filter team member listings
    ///
    /// # Filter parameters
    /// - _q
    /// - id
    /// - user_id
    /// - username
    /// - level
    /// - date_added
    /// - position
    ///
    /// # Sorting
    /// - id
    /// - user_id
    /// - username
    ///
    /// See [modio docs](https://docs.mod.io/#get-all-mod-team-members) for more information.
    ///
    /// By default this returns up to `100` items. You can limit the result using `limit` and
    /// `offset`.
    /// # Example
    /// ```
    /// use modio::filter::{Order, Operator};
    /// use modio::teams::TeamMemberListOptions;
    ///
    /// let mut opts = TeamMemberListOptions::new();
    /// opts.id(Operator::In, vec![1, 2]);
    /// opts.sort_by(TeamMemberListOptions::ID, Order::Desc);
    /// ```
    #[derive(Debug)]
    pub struct TeamMemberListOptions {
        Filters
        - id = "id";
        - user_id = "user_id";
        - username = "username";
        - level = "level";
        - date_added = "date_added";
        - position = "position";

        Sort
        - ID = "id";
        - USER_ID = "user_id";
        - USERNAME = "username";
    }
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
