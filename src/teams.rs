use std::collections::HashMap;

use futures::future;
use hyper::client::Connect;
use serde_urlencoded;
use url::form_urlencoded;

use types::mods::{TeamLevel, TeamMember};
use types::{ModioListResponse, ModioMessage};
use Future;
use Modio;

pub struct Members<C>
where
    C: Clone + Connect,
{
    modio: Modio<C>,
    game: u32,
    mod_id: u32,
}

impl<C: Clone + Connect> Members<C> {
    pub fn new(modio: Modio<C>, game: u32, mod_id: u32) -> Self {
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
        if let Some(query) = options.serialize() {
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

#[derive(Default)]
pub struct TeamMemberListOptions {
    params: HashMap<&'static str, String>,
}

impl TeamMemberListOptions {
    pub fn serialize(&self) -> Option<String> {
        if self.params.is_empty() {
            None
        } else {
            let encoded = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(&self.params)
                .finish();
            Some(encoded)
        }
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
