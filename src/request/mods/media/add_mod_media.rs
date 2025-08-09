use std::ffi::OsStr;
use std::future::IntoFuture;
use std::path::Path;

use crate::client::Client;
use crate::request::multipart::{Form, Part};
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::id::{GameId, ModId};
use crate::types::Message;

/// Add images, sketchfab or youtube links to a mod profile.
pub struct AddModMedia<'a> {
    http: &'a Client,
    game_id: GameId,
    mod_id: ModId,
    fields: AddModMediaFields<'a>,
}

struct AddModMediaFields<'a> {
    sync: Option<bool>,
    logo: Option<Part>,
    images: Vec<Part>,
    images_zip: Option<Part>,
    youtube: Option<&'a [&'a str]>,
    sketchfab: Option<&'a [&'a str]>,
}

impl<'a> AddModMedia<'a> {
    pub(crate) const fn new(http: &'a Client, game_id: GameId, mod_id: ModId) -> Self {
        Self {
            http,
            game_id,
            mod_id,
            fields: AddModMediaFields {
                sync: None,
                logo: None,
                images: Vec::new(),
                images_zip: None,
                youtube: None,
                sketchfab: None,
            },
        }
    }

    pub const fn sync(mut self, value: bool) -> Self {
        self.fields.sync = Some(value);
        self
    }

    pub fn logo<P: AsRef<Path>>(mut self, file: P) -> Self {
        let part = Part::file(file, "logo.png").mime(mime::IMAGE_STAR);
        self.fields.logo = Some(part);
        self
    }

    pub fn images_zip<P: AsRef<Path>>(mut self, file: P) -> Self {
        let part = Part::file(file, "images.zip").mime(mime::APPLICATION_OCTET_STREAM);
        self.fields.images_zip = Some(part);
        self
    }

    pub fn images<P: AsRef<Path>>(mut self, images: &'a [P]) -> Self {
        self.fields.images = images
            .iter()
            .enumerate()
            .map(|(i, file)| {
                let file = file.as_ref();
                let filename = file
                    .file_name()
                    .and_then(OsStr::to_str)
                    .map_or_else(|| format!("image{i}.png"), ToString::to_string);

                Part::file(file, &filename).mime(mime::IMAGE_STAR)
            })
            .collect();
        self
    }

    pub const fn youtube(mut self, youtube: &'a [&'a str]) -> Self {
        self.fields.youtube = Some(youtube);
        self
    }

    pub const fn sketchfab(mut self, sketchfab: &'a [&'a str]) -> Self {
        self.fields.sketchfab = Some(sketchfab);
        self
    }
}

impl IntoFuture for AddModMedia<'_> {
    type Output = Output<Message>;
    type IntoFuture = ResponseFuture<Message>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::AddModMedia {
            game_id: self.game_id,
            mod_id: self.mod_id,
        };
        let mut form = Form::new();

        if let Some(sync) = self.fields.sync {
            form = form.text("sync", sync.to_string());
        }
        if let Some(part) = self.fields.logo {
            form = form.part("logo", part);
        }
        for (i, part) in self.fields.images.into_iter().enumerate() {
            form = form.part(format!("image{i}"), part);
        }
        if let Some(part) = self.fields.images_zip {
            form = form.part("images", part);
        }
        if let Some(youtube) = self.fields.youtube {
            for part in youtube {
                form = form.text("youtube[]", (*part).to_owned());
            }
        }
        if let Some(sketchfab) = self.fields.sketchfab {
            for part in sketchfab {
                form = form.text("sketchfab[]", (*part).to_owned());
            }
        }

        match RequestBuilder::from_route(&route).multipart(form) {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}
