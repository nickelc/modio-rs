use std::future::IntoFuture;
use std::path::Path;

use crate::client::Client;
use crate::request::multipart::{Form, Part};
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::id::GameId;
use crate::types::Message;

/// Update new media to a game.
pub struct AddGameMedia<'a> {
    http: &'a Client,
    game_id: GameId,
    logo: Option<Part>,
    icon: Option<Part>,
    header: Option<Part>,
}

impl<'a> AddGameMedia<'a> {
    pub(crate) const fn new(http: &'a Client, game_id: GameId) -> Self {
        Self {
            http,
            game_id,
            logo: None,
            icon: None,
            header: None,
        }
    }

    pub fn logo<P: AsRef<Path>>(mut self, file: P) -> Self {
        let part = Part::file(file, "logo.png").mime(mime::IMAGE_STAR);
        self.logo = Some(part);
        self
    }

    pub fn icon<P: AsRef<Path>>(mut self, file: P) -> Self {
        let part = Part::file(file, "icon.png").mime(mime::IMAGE_STAR);
        self.icon = Some(part);
        self
    }

    pub fn header<P: AsRef<Path>>(mut self, file: P) -> Self {
        let part = Part::file(file, "header.png").mime(mime::IMAGE_STAR);
        self.header = Some(part);
        self
    }
}

impl IntoFuture for AddGameMedia<'_> {
    type Output = Output<Message>;
    type IntoFuture = ResponseFuture<Message>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::AddGameMedia {
            game_id: self.game_id,
        };

        let mut form = Form::new();
        if let Some(value) = self.logo {
            form = form.part("logo", value);
        }
        if let Some(value) = self.icon {
            form = form.part("icon", value);
        }
        if let Some(value) = self.header {
            form = form.part("header", value);
        }

        match RequestBuilder::from_route(&route).multipart(form) {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}
