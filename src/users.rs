use std::collections::HashMap;

use hyper::client::connect::Connect;
use url::form_urlencoded;

use types::ModioListResponse;
use types::User;
use Future;
use Modio;
use QueryParams;

pub struct Users<C>
where
    C: Clone + Connect + 'static,
{
    modio: Modio<C>,
}

impl<C: Clone + Connect + 'static> Users<C> {
    pub fn new(modio: Modio<C>) -> Self {
        Self { modio }
    }

    pub fn list(&self, options: &UsersListOptions) -> Future<ModioListResponse<User>> {
        let mut uri = vec!["/users".into()];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.modio.get(&uri.join("?"))
    }

    pub fn get(&self, id: u32) -> Future<User> {
        self.modio.get(&format!("/users/{}", id))
    }

    pub fn get_owner(&self, resource: Resource) -> Future<User> {
        let params = resource.to_query_params();
        self.modio.post("/general/ownership", params)
    }
}

pub enum Resource {
    Game(u32),
    Mod(u32),
    File(u32),
}

impl QueryParams for Resource {
    fn to_query_params(&self) -> String {
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

#[derive(Default)]
pub struct UsersListOptions {
    params: HashMap<&'static str, String>,
}

impl UsersListOptions {
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
