//! Users interface

use url::form_urlencoded;

use crate::prelude::*;

pub use crate::types::{Avatar, User};

/// Interface for users.
pub struct Users<C>
where
    C: Clone + Connect + 'static,
{
    modio: Modio<C>,
}

impl<C: Clone + Connect + 'static> Users<C> {
    pub(crate) fn new(modio: Modio<C>) -> Self {
        Self { modio }
    }

    /// List all users registered on [mod.io](https:://mod.io).
    pub fn list(&self, options: &UsersListOptions) -> Future<List<User>> {
        let mut uri = vec!["/users".into()];
        let query = options.to_query_params();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.get(&uri.join("?"))
    }

    /// Provides a stream over all users registered on [mod.io](https:://mod.io).
    pub fn iter(&self, options: &UsersListOptions) -> Stream<User> {
        let mut uri = vec!["/users".into()];
        let query = options.to_query_params();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.stream(&uri.join("?"))
    }

    /// Return a user by id
    pub fn get(&self, id: u32) -> Future<User> {
        self.modio.get(&format!("/users/{}", id))
    }

    /// Return the user that is the original submitter of a resource.
    pub fn get_owner(&self, resource: Resource) -> Future<User> {
        let params = resource.to_query_params();
        self.modio.post("/general/ownership", params)
    }
}

#[derive(Clone, Copy)]
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

filter_options! {
    /// Options used to filter user listings
    ///
    /// # Filter parameters
    /// - _q
    /// - id
    /// - name_id
    /// - level
    /// - date_online
    /// - username
    /// - timezone
    /// - language
    ///
    /// # Sorting
    /// - id
    /// - username
    ///
    /// See [modio docs](https://docs.mod.io/#get-all-users) for more information.
    ///
    /// By default this returns up to `100` items. You can limit the result using `limit` and
    /// `offset`.
    /// # Example
    /// ```
    /// use modio::filter::{Order, Operator};
    /// use modio::users::UsersListOptions;
    ///
    /// let mut opts = UsersListOptions::new();
    /// opts.id(Operator::In, vec![1, 2]);
    /// opts.sort_by(UsersListOptions::ID, Order::Desc);
    /// ```
    #[derive(Debug)]
    pub struct UsersListOptions {
        Filters
        - id = "id";
        - name_id = "name_id";
        - date_online = "date_online";
        - username = "username";
        - timezone = "timezone";
        - language = "language";

        Sort
        - ID = "id";
        - USERNAME = "username";
    }
}
