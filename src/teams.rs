//! Team members interface
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

    /// Returns a `Query` interface to retrieve all team members.
    ///
    /// See [Filters and sorting](filters).
    pub fn search(&self, filter: Filter) -> Query<TeamMember> {
        let route = Route::GetTeamMembers {
            game_id: self.game,
            mod_id: self.mod_id,
        };
        Query::new(self.modio.clone(), route, filter)
    }

    /// Add a team member by email. [required: token]
    #[allow(clippy::should_implement_trait)]
    pub async fn add(self, options: InviteTeamMemberOptions) -> Result<()> {
        let route = Route::AddTeamMember {
            game_id: self.game,
            mod_id: self.mod_id,
        };
        self.modio
            .request(route)
            .form(&options)
            .send::<Message>()
            .await?;

        Ok(())
    }

    /// Edit a team member by id. [required: token]
    pub async fn edit(self, id: u32, options: EditTeamMemberOptions) -> Result<()> {
        let route = Route::EditTeamMember {
            game_id: self.game,
            mod_id: self.mod_id,
            member_id: id,
        };
        self.modio
            .request(route)
            .form(&options)
            .send::<Message>()
            .await?;

        Ok(())
    }

    /// Delete a team member by id. [required: token]
    pub async fn delete(self, id: u32) -> Result<()> {
        let route = Route::DeleteTeamMember {
            game_id: self.game,
            mod_id: self.mod_id,
            member_id: id,
        };
        self.modio.request(route).send().await?;
        Ok(())
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

impl_serialize_params!(InviteTeamMemberOptions >> params);

#[derive(Debug, Default)]
pub struct EditTeamMemberOptions {
    params: std::collections::BTreeMap<&'static str, String>,
}

impl EditTeamMemberOptions {
    option!(level: TeamLevel >> "level");
    option!(position >> "position");
}

impl_serialize_params!(EditTeamMemberOptions >> params);
