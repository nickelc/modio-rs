//! Users interface
use url::form_urlencoded;

use crate::prelude::*;

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
    pub async fn get_owner(self, resource: Resource) -> Result<User> {
        self.modio
            .request(Route::GetResourceOwner)
            .body(resource.to_query_string())
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
