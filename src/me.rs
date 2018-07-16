//! Me interface

use hyper::client::connect::Connect;

use files::MyFiles;
use games::MyGames;
use mods::MyMods;
use types::mods::Mod;
use types::Event;
use types::User;
use Future;
use Modio;
use ModioListResponse;

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
    pub fn user(&self) -> Future<User> {
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
    pub fn events(&self) -> Future<ModioListResponse<Event>> {
        self.modio.get("/me/events")
    }

    /// Return all mod's the authenticated user is subscribed to.
    pub fn subscriptions(&self) -> Future<ModioListResponse<Mod>> {
        self.modio.get("/me/subscribed")
    }
}
