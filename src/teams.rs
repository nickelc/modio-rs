use futures::future;
use hyper::client::connect::Connect;
use serde_urlencoded;

use filter::{Filter, OneOrMany, Operator, Order, SortField};
use types::mods::{TeamLevel, TeamMember};
use types::{ModioListResponse, ModioMessage};
use Future;
use Modio;
use QueryParams;

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

    pub fn list(&self, options: &TeamMemberListOptions) -> Future<ModioListResponse<TeamMember>> {
        let mut uri = vec![self.path("")];
        let query = options.to_query_params();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.get(&uri.join("&"))
    }

    pub fn add(&self, options: &InviteTeamMemberOptions) -> Future<ModioMessage> {
        let msg = match serde_urlencoded::to_string(&options) {
            Ok(data) => data,
            Err(err) => return Box::new(future::err(err.into())),
        };
        self.modio.post(&self.path(""), msg)
    }

    pub fn edit(&self, id: u32, options: &EditTeamMemberOptions) -> Future<ModioMessage> {
        let msg = match serde_urlencoded::to_string(&options) {
            Ok(data) => data,
            Err(err) => return Box::new(future::err(err.into())),
        };
        self.modio.put(&self.path(&format!("/{}", id)), msg)
    }

    pub fn delete(&self, id: u32) -> Future<()> {
        self.modio
            .delete(&self.path(&format!("/{}", id)), Vec::new())
    }
}

filter_options!{
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
    /// See [modio docs](https://docs.mod.io/#get-all-mod-team-members) for more informations.
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

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
pub struct EditTeamMemberOptions {
    level: Option<TeamLevel>,
    position: Option<String>,
}

impl EditTeamMemberOptions {
    pub fn builder() -> EditTeamMemberOptionsBuilder {
        EditTeamMemberOptionsBuilder::new()
    }
}

pub struct EditTeamMemberOptionsBuilder(EditTeamMemberOptions);

impl EditTeamMemberOptionsBuilder {
    pub fn new() -> Self {
        EditTeamMemberOptionsBuilder(EditTeamMemberOptions {
            level: None,
            position: None,
        })
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
