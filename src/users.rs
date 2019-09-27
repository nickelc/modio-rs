//! Users interface
use url::form_urlencoded;

use crate::prelude::*;

use crate::types::game::Game;
use crate::types::mods::{File, Mod};
pub use crate::types::{Avatar, User};

/// Interface for users.
pub struct Users {
    modio: Modio,
}

impl Users {
    pub(crate) fn new(modio: Modio) -> Self {
        Self { modio }
    }

    /// Return the user that is the original submitter of a resource. [required: token]
    pub async fn get_owner<T>(self, resource: T) -> Result<User>
    where
        Resource: From<T>,
    {
        self.modio
            .request(Route::GetResourceOwner)
            .body(Resource::from(resource).to_query_string())
            .send()
            .await
    }
}

#[derive(Clone, Copy)]
pub enum Resource {
    Game(u32),
    Mod(u32),
    File(u32),
}

impl crate::private::Sealed for Resource {}

impl QueryString for Resource {
    fn to_query_string(&self) -> String {
        let (_type, id) = match *self {
            Resource::Game(id) => ("games", id),
            Resource::Mod(id) => ("mods", id),
            Resource::File(id) => ("files", id),
        };
        form_urlencoded::Serializer::new(String::new())
            .append_pair("resource_type", _type)
            .append_pair("resource_id", &id.to_string())
            .finish()
    }
}

impl From<Game> for Resource {
    fn from(game: Game) -> Resource {
        Resource::Game(game.id)
    }
}

impl From<&Game> for Resource {
    fn from(game: &Game) -> Resource {
        Resource::Game(game.id)
    }
}

impl From<Mod> for Resource {
    fn from(m: Mod) -> Resource {
        Resource::Mod(m.id)
    }
}

impl From<&Mod> for Resource {
    fn from(m: &Mod) -> Resource {
        Resource::Mod(m.id)
    }
}

impl From<File> for Resource {
    fn from(file: File) -> Resource {
        Resource::File(file.id)
    }
}

impl From<&File> for Resource {
    fn from(file: &File) -> Resource {
        Resource::File(file.id)
    }
}
