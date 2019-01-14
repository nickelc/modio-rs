//! Me interface

use hyper::client::connect::Connect;

use crate::files::MyFiles;
use crate::games::MyGames;
use crate::mods::MyMods;
use crate::types::mods::Mod;
use crate::types::Event;
use crate::types::User;
use crate::EventListOptions;
use crate::Future;
use crate::Modio;
use crate::ModioListResponse;
use crate::QueryParams;

pub use crate::types::mods::Rating;

/// Interface for resources owned by the authenticated user or is team member of.
pub struct Me<C>
where
    C: Clone + Connect + 'static,
{
    modio: Modio<C>,
}

impl<C: Clone + Connect + 'static> Me<C> {
    pub(crate) fn new(modio: Modio<C>) -> Self {
        Self { modio }
    }

    /// Return the authenticated user.
    pub fn authenticated_user(&self) -> Future<User> {
        self.modio.get("/me")
    }

    /// Return a reference to an interface that provides access to games the authenticated user
    /// added or is a team member of.
    pub fn games(&self) -> MyGames<C> {
        MyGames::new(self.modio.clone())
    }

    /// Return a reference to an interface that provides access to mods the authenticated user
    /// added or is a team member of.
    pub fn mods(&self) -> MyMods<C> {
        MyMods::new(self.modio.clone())
    }

    /// Return a reference to an interface that provides access to modfiles the authenticated user
    /// uploaded.
    pub fn files(&self) -> MyFiles<C> {
        MyFiles::new(self.modio.clone())
    }

    /// Return the events that have been fired specific to the authenticated user.
    pub fn events(&self, options: &EventListOptions) -> Future<ModioListResponse<Event>> {
        let mut uri = vec!["/me/events".to_owned()];
        let query = options.to_query_params();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.get(&uri.join("?"))
    }

    /// Return all mod's the authenticated user is subscribed to.
    pub fn subscriptions(
        &self,
        options: &SubscriptionsListOptions,
    ) -> Future<ModioListResponse<Mod>> {
        let mut uri = vec!["/me/subscribed".to_owned()];
        let query = options.to_query_params();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.get(&uri.join("?"))
    }

    /// Return all mod rating's submitted by the authenticated user.
    pub fn ratings(&self, options: &RatingsListOptions) -> Future<ModioListResponse<Rating>> {
        let mut uri = vec!["/me/ratings".to_owned()];
        let query = options.to_query_params();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.get(&uri.join("?"))
    }
}

filter_options! {
    /// Options used to filter subscription listings.
    ///
    /// # Filter parameters
    /// - _q
    /// - id
    /// - game_id
    /// - submitted_by
    /// - date_added
    /// - date_updated
    /// - date_live
    /// - name
    /// - name_id
    /// - summary
    /// - description
    /// - homepage_url
    /// - metadata_blob
    /// - tags
    ///
    /// # Sorting
    /// - id
    /// - name
    /// - downloads
    /// - popular
    /// - ratings
    /// - subscribers
    ///
    /// See the [mod.io docs](https://docs.mod.io/#get-user-subscriptions) for more informations.
    ///
    /// By default this returns up to `100` items. You can limit the result using `limit` and
    /// `offset`.
    /// # Example
    /// ```
    /// use modio::filter::{Order, Operator};
    /// use modio::me::SubscriptionsListOptions;
    ///
    /// let mut opts = SubscriptionsListOptions::new();
    /// opts.game_id(Operator::In, vec![1, 2]);
    /// opts.sort_by(SubscriptionsListOptions::DATE_UPDATED, Order::Desc);
    /// ```
    #[derive(Debug)]
    pub struct SubscriptionsListOptions {
        Filters
        - id = "id";
        - game_id = "game_id";
        - submitted_by = "submitted_by";
        - date_added = "date_added";
        - date_updated = "date_updated";
        - date_live = "date_live";
        - name = "name";
        - name_id = "name_id";
        - summary = "summary";
        - description = "description";
        - homepage_url = "homepage_url";
        - metadata_blob = "metadata_blob";
        - tags = "tags";

        Sort
        - ID = "id";
        - GAME_ID = "game_id";
        - DATE_UPDATED = "date_updated";
        - NAME = "name";
        - DOWNLOADS = "downloads";
        - POPULAR = "popular";
        - RATINGS = "ratings";
        - SUBSCRIBERS = "subscribers";
    }
}

filter_options! {
    /// Options used to filter rating listings.
    ///
    /// # Filter parameters
    /// - _q
    /// - game_id
    /// - mod_id
    /// - date_added
    ///
    /// See the [mod.io docs](https://docs.mod.io/#get-user-ratings) for more informations.
    ///
    /// By default this returns up to `100` items. You can limit the result using `limit` and
    /// `offset`.
    /// # Example
    /// ```
    /// use modio::filter::{Order, Operator};
    /// use modio::me::RatingsListOptions;
    ///
    /// let mut opts = RatingsListOptions::new();
    /// opts.game_id(Operator::In, vec![1, 2]);
    /// opts.sort_by(RatingsListOptions::DATE_ADDED, Order::Desc);
    /// ```
    #[derive(Debug)]
    pub struct RatingsListOptions {
        Filters
        - game_id = "game_id";
        - mod_id = "mod_id";
        - date_added = "date_added";

        Sort
        - GAME_ID = "game_id";
        - MOD_ID = "mod_id";
        - DATE_ADDED = "date_added";
    }
}
