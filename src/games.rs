//! Games interface
use std::ffi::OsStr;
use std::path::Path;

use mime::IMAGE_STAR;
use url::form_urlencoded;

use crate::error::Result;
use crate::multipart::FileSource;
use crate::prelude::*;
use crate::ModRef;
use crate::Mods;

pub use crate::types::game::{
    ApiAccessOptions, CommunityOptions, CurationOption, Game, HeaderImage, Icon, MaturityOptions,
    PresentationOption, RevenueOptions, SubmissionOption, TagOption, TagType,
};
pub use crate::types::Logo;
pub use crate::types::Status;

/// Interface for games the authenticated user added or is team member of.
pub struct MyGames {
    modio: Modio,
}

impl MyGames {
    pub(crate) fn new(modio: Modio) -> Self {
        Self { modio }
    }

    /// List all games the authenticated user added or is team member of. [required: token]
    ///
    /// See [Filters and sorting](filters/index.html).
    pub async fn list(self, filter: Filter) -> Result<List<Game>> {
        self.modio
            .request(Route::UserGames)
            .query(filter.to_query_string())
            .send()
            .await
    }

    /// Provides a stream over all games the authenticated user added or is team member of.
    /// [required: token]
    ///
    /// See [Filters and sorting](filters/index.html).
    pub fn iter<'a>(self, filter: Filter) -> Stream<'a, Game> {
        self.modio.stream(Route::UserGames, filter)
    }
}

/// Interface for games.
pub struct Games {
    modio: Modio,
}

impl Games {
    pub(crate) fn new(modio: Modio) -> Self {
        Self { modio }
    }

    /// List all games.
    ///
    /// See [Filters and sorting](filters/index.html).
    pub async fn list(self, filter: Filter) -> Result<List<Game>> {
        self.modio
            .request(Route::GetGames)
            .query(filter.to_query_string())
            .send()
            .await
    }

    /// Provides a stream over all games.
    ///
    /// See [Filters and sorting](filters/index.html).
    pub fn iter<'a>(self, filter: Filter) -> Stream<'a, Game> {
        self.modio.stream(Route::GetGames, filter)
    }

    /// Return a reference to a game.
    pub fn get(&self, id: u32) -> GameRef {
        GameRef::new(self.modio.clone(), id)
    }
}

/// Reference interface of a game.
pub struct GameRef {
    modio: Modio,
    id: u32,
}

impl GameRef {
    pub(crate) fn new(modio: Modio, id: u32) -> Self {
        Self { modio, id }
    }

    /// Get a reference to the Modio game object that this `GameRef` refers to.
    pub async fn get(self) -> Result<Game> {
        let route = Route::GetGame { game_id: self.id };
        self.modio.request(route).send().await
    }

    /// Return a reference to a mod of a game.
    pub fn mod_(&self, mod_id: u32) -> ModRef {
        ModRef::new(self.modio.clone(), self.id, mod_id)
    }

    /// Return a reference to an interface that provides access to the mods of a game.
    pub fn mods(&self) -> Mods {
        Mods::new(self.modio.clone(), self.id)
    }

    /// Return a reference to an interface that provides access to the tags of a game.
    pub fn tags(&self) -> Endpoint<TagOption> {
        let list = Route::GetGameTags { game_id: self.id };
        let add = Route::AddGameTags { game_id: self.id };
        let delete = Route::DeleteGameTags { game_id: self.id };
        Endpoint::new(self.modio.clone(), list, add, delete)
    }

    /// Edit details for a game. [required: token]
    pub async fn edit(self, options: EditGameOptions) -> Result<EntityResult<Game>> {
        let route = Route::EditGame { game_id: self.id };
        self.modio
            .request(route)
            .body(options.to_query_string())
            .send()
            .await
    }

    /// Add or edit new media to a game. [required: token]
    pub async fn add_media(self, media: GameMediaOptions) -> Result<()> {
        let route = Route::AddGameMedia { game_id: self.id };
        self.modio
            .request(route)
            .body(Form::from(media))
            .send::<ModioMessage>()
            .await?;
        Ok(())
    }
}

/// Game filters and sorting.
///
/// # Filters
/// - Fulltext
/// - Id
/// - Status
/// - SubmittedBy
/// - DateAdded
/// - DateUpdated
/// - DateLive
/// - Name
/// - NameId
/// - Summary
/// - InstructionsUrl
/// - UgcName
/// - PresentationOption
/// - SubmissionOption
/// - CurationOption
/// - CommunityOptions
/// - RevenueOptions
/// - ApiAccessOptions
/// - MaturityOptions
///
/// # Sorting
/// - Id
/// - Status
/// - Name
/// - NameId
/// - DateUpdated
///
/// See [modio docs](https://docs.mod.io/#get-all-games) for more information.
///
/// By default this returns up to `100` items. You can limit the result by using `limit` and
/// `offset`.
///
/// # Example
/// ```
/// use modio::filter::prelude::*;
/// use modio::games::filters::Id;
///
/// let filter = Id::_in(vec![1, 2]).order_by(Id::desc());
/// ```
#[rustfmt::skip]
pub mod filters {
    #[doc(inline)]
    pub use crate::filter::prelude::Fulltext;
    #[doc(inline)]
    pub use crate::filter::prelude::Id;
    #[doc(inline)]
    pub use crate::filter::prelude::Name;
    #[doc(inline)]
    pub use crate::filter::prelude::NameId;
    #[doc(inline)]
    pub use crate::filter::prelude::Status;
    #[doc(inline)]
    pub use crate::filter::prelude::DateAdded;
    #[doc(inline)]
    pub use crate::filter::prelude::DateUpdated;
    #[doc(inline)]
    pub use crate::filter::prelude::DateLive;
    #[doc(inline)]
    pub use crate::filter::prelude::SubmittedBy;

    filter!(Summary, SUMMARY, "summary", Eq, NotEq, Like);
    filter!(InstructionsUrl, INSTRUCTIONS_URL, "instructions_url", Eq, NotEq, In, Like);
    filter!(UgcName, UGC_NAME, "ugc_name", Eq, NotEq, In, Like);
    filter!(PresentationOption, PRESENTATION_OPTION, "presentation_option", Eq, NotEq, In, Cmp, Bit);
    filter!(SubmissionOption, SUBMISSION_OPTION, "submission_option", Eq, NotEq, In, Cmp, Bit);
    filter!(CurationOption, CURATION_OPTION, "curation_option", Eq, NotEq, In, Cmp, Bit);
    filter!(CommunityOptions, COMMUNITY_OPTIONS, "community_options", Eq, NotEq, In, Cmp, Bit);
    filter!(RevenueOptions, REVENUE_OPTIONS, "revenue_options", Eq, NotEq, In, Cmp, Bit);
    filter!(ApiAccessOptions, API_ACCESS_OPTIONS, "api_access_options", Eq, NotEq, In, Cmp, Bit);
    filter!(MaturityOptions, MATURITY_OPTIONS, "maturity_options", Eq, NotEq, In, Cmp, Bit);
}

#[derive(Default)]
pub struct EditGameOptions {
    params: std::collections::BTreeMap<&'static str, String>,
}

impl EditGameOptions {
    option!(status: Status >> "status");
    option!(name >> "name");
    option!(name_id >> "name_id");
    option!(summary >> "summary");
    option!(instructions >> "instructions");
    option!(instructions_url >> "instructions_url");
    option!(ugc_name >> "ugc_name");
    option!(presentation_option: PresentationOption >> "presentation_option");
    option!(submission_option: SubmissionOption >> "submission_option");
    option!(curation_option: CurationOption >> "curation_option");
    option!(community_options: CommunityOptions >> "community_options");
    option!(revenue_options: RevenueOptions >> "revenue_options");
    option!(api_access_options: ApiAccessOptions >> "api_access_options");
    option!(maturity_options: MaturityOptions >> "maturity_options");
}

impl crate::QueryString for EditGameOptions {
    fn to_query_string(&self) -> String {
        url::form_urlencoded::Serializer::new(String::new())
            .extend_pairs(&self.params)
            .finish()
    }
}

pub struct AddTagsOptions {
    name: String,
    kind: TagType,
    hidden: bool,
    tags: Vec<String>,
}

impl AddTagsOptions {
    pub fn public<S: Into<String>>(name: S, kind: TagType, tags: &[String]) -> Self {
        Self {
            name: name.into(),
            kind,
            hidden: false,
            tags: tags.to_vec(),
        }
    }

    pub fn hidden<S: Into<String>>(name: S, kind: TagType, tags: &[String]) -> Self {
        Self {
            name: name.into(),
            kind,
            hidden: true,
            tags: tags.to_vec(),
        }
    }
}

impl AddOptions for AddTagsOptions {}

impl QueryString for AddTagsOptions {
    fn to_query_string(&self) -> String {
        form_urlencoded::Serializer::new(String::new())
            .append_pair("name", &self.name)
            .append_pair("type", &self.kind.to_string())
            .append_pair("hidden", &self.hidden.to_string())
            .extend_pairs(self.tags.iter().map(|t| ("tags[]", t)))
            .finish()
    }
}

pub struct DeleteTagsOptions {
    name: String,
    tags: Option<Vec<String>>,
}

impl DeleteTagsOptions {
    pub fn all<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            tags: None,
        }
    }

    pub fn some<S: Into<String>>(name: S, tags: &[String]) -> Self {
        Self {
            name: name.into(),
            tags: if tags.is_empty() {
                None
            } else {
                Some(tags.to_vec())
            },
        }
    }
}

impl DeleteOptions for DeleteTagsOptions {}

impl QueryString for DeleteTagsOptions {
    fn to_query_string(&self) -> String {
        let mut ser = form_urlencoded::Serializer::new(String::new());
        ser.append_pair("name", &self.name);
        match &self.tags {
            Some(tags) => ser.extend_pairs(tags.iter().map(|t| ("tags[]", t))),
            None => ser.append_pair("tags[]", ""),
        };
        ser.finish()
    }
}

#[derive(Default)]
pub struct GameMediaOptions {
    logo: Option<FileSource>,
    icon: Option<FileSource>,
    header: Option<FileSource>,
}

impl GameMediaOptions {
    pub fn logo<P: AsRef<Path>>(self, logo: P) -> Self {
        let logo = logo.as_ref();
        let filename = logo
            .file_name()
            .and_then(OsStr::to_str)
            .map_or_else(String::new, ToString::to_string);

        Self {
            logo: Some(FileSource::new_from_file(logo, filename, IMAGE_STAR)),
            ..self
        }
    }

    pub fn icon<P: AsRef<Path>>(self, icon: P) -> Self {
        let icon = icon.as_ref();
        let filename = icon
            .file_name()
            .and_then(OsStr::to_str)
            .map_or_else(String::new, ToString::to_string);

        Self {
            icon: Some(FileSource::new_from_file(icon, filename, IMAGE_STAR)),
            ..self
        }
    }

    pub fn header<P: AsRef<Path>>(self, header: P) -> Self {
        let header = header.as_ref();
        let filename = header
            .file_name()
            .and_then(OsStr::to_str)
            .map_or_else(String::new, ToString::to_string);

        Self {
            header: Some(FileSource::new_from_file(header, filename, IMAGE_STAR)),
            ..self
        }
    }
}

#[doc(hidden)]
impl From<GameMediaOptions> for Form {
    fn from(opts: GameMediaOptions) -> Form {
        let mut form = Form::new();
        if let Some(logo) = opts.logo {
            form = form.part("logo", logo.into());
        }
        if let Some(icon) = opts.icon {
            form = form.part("icon", icon.into());
        }
        if let Some(header) = opts.header {
            form = form.part("header", header.into());
        }
        form
    }
}
