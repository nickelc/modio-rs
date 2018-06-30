use hyper::client::Connect;

use files::MyFiles;
use games::MyGames;
use mods::MyMods;
use types::mods::Mod;
use types::Event;
use types::User;
use Future;
use Modio;
use ModioListResponse;

pub struct Me<C>
where
    C: Clone + Connect + 'static,
{
    modio: Modio<C>,
}

impl<C: Clone + Connect + 'static> Me<C> {
    pub fn new(modio: Modio<C>) -> Self {
        Self { modio }
    }

    pub fn user(&self) -> Future<User> {
        self.modio.get("/me")
    }

    pub fn games(&self) -> MyGames<C> {
        MyGames::new(self.modio.clone())
    }

    pub fn mods(&self) -> MyMods<C> {
        MyMods::new(self.modio.clone())
    }

    pub fn files(&self) -> MyFiles<C> {
        MyFiles::new(self.modio.clone())
    }

    pub fn events(&self) -> Future<ModioListResponse<Event>> {
        self.modio.get("/me/events")
    }

    pub fn subscriptions(&self) -> Future<ModioListResponse<Mod>> {
        self.modio.get("/me/subscribed")
    }
}
