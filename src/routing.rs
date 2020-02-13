use std::borrow::Cow;

use reqwest::Method;

#[derive(Clone, Debug)]
pub enum Route {
    AuthEmailRequest,
    AuthEmailExchange,
    AuthSteam,
    AuthGog,
    AuthOculus,
    LinkAccount,
    GetGames,
    GetGame {
        game_id: u32,
    },
    AddGameMedia {
        game_id: u32,
    },
    EditGame {
        game_id: u32,
    },
    GetGameTags {
        game_id: u32,
    },
    AddGameTags {
        game_id: u32,
    },
    DeleteGameTags {
        game_id: u32,
    },
    GetMods {
        game_id: u32,
    },
    GetMod {
        game_id: u32,
        mod_id: u32,
    },
    AddMod {
        game_id: u32,
    },
    EditMod {
        game_id: u32,
        mod_id: u32,
    },
    DeleteMod {
        game_id: u32,
        mod_id: u32,
    },
    AddModMedia {
        game_id: u32,
        mod_id: u32,
    },
    DeleteModMedia {
        game_id: u32,
        mod_id: u32,
    },
    Subscribe {
        game_id: u32,
        mod_id: u32,
    },
    Unsubscribe {
        game_id: u32,
        mod_id: u32,
    },
    GetAllModEvents {
        game_id: u32,
    },
    GetModEvents {
        game_id: u32,
        mod_id: u32,
    },
    GetAllModStats {
        game_id: u32,
    },
    GetModStats {
        game_id: u32,
        mod_id: u32,
    },
    GetModTags {
        game_id: u32,
        mod_id: u32,
    },
    AddModTags {
        game_id: u32,
        mod_id: u32,
    },
    DeleteModTags {
        game_id: u32,
        mod_id: u32,
    },
    RateMod {
        game_id: u32,
        mod_id: u32,
    },
    GetModMetadata {
        game_id: u32,
        mod_id: u32,
    },
    AddModMetadata {
        game_id: u32,
        mod_id: u32,
    },
    DeleteModMetadata {
        game_id: u32,
        mod_id: u32,
    },
    GetModDependencies {
        game_id: u32,
        mod_id: u32,
    },
    AddModDepencencies {
        game_id: u32,
        mod_id: u32,
    },
    DeleteModDependencies {
        game_id: u32,
        mod_id: u32,
    },
    GetTeamMembers {
        game_id: u32,
        mod_id: u32,
    },
    AddTeamMember {
        game_id: u32,
        mod_id: u32,
    },
    EditTeamMember {
        game_id: u32,
        mod_id: u32,
        member_id: u32,
    },
    DeleteTeamMember {
        game_id: u32,
        mod_id: u32,
        member_id: u32,
    },
    GetModComments {
        game_id: u32,
        mod_id: u32,
    },
    GetModComment {
        game_id: u32,
        mod_id: u32,
        comment_id: u32,
    },
    DeleteModComment {
        game_id: u32,
        mod_id: u32,
        comment_id: u32,
    },
    GetFiles {
        game_id: u32,
        mod_id: u32,
    },
    GetFile {
        game_id: u32,
        mod_id: u32,
        file_id: u32,
    },
    AddFile {
        game_id: u32,
        mod_id: u32,
    },
    EditFile {
        game_id: u32,
        mod_id: u32,
        file_id: u32,
    },
    DeleteFile {
        game_id: u32,
        mod_id: u32,
        file_id: u32,
    },
    AuthorizedUser,
    UserSubscriptions,
    UserEvents,
    UserGames,
    UserMods,
    UserFiles,
    UserRatings,
    SubmitReport,
}

pub enum AuthMethod {
    ApiKey,
    Token,
    Any,
}

macro_rules! route {
    ($method:ident, $route:expr, $required:ident) => {
        (Method::$method, AuthMethod::$required, Cow::from($route))
    };
}

impl Route {
    pub fn pieces(&self) -> (Method, AuthMethod, Cow<'_, str>) {
        use Route::*;

        match *self {
            AuthEmailRequest => route!(POST, Route::auth_email_request(), ApiKey),
            AuthEmailExchange => route!(POST, Route::auth_email_exchange(), ApiKey),
            AuthSteam => route!(POST, Route::auth_steam(), ApiKey),
            AuthGog => route!(POST, Route::auth_gog(), ApiKey),
            AuthOculus => route!(POST, Route::auth_oculus(), ApiKey),
            LinkAccount => route!(POST, Route::link_account(), Token),
            GetGames => route!(GET, Route::games(), Any),
            GetGame { game_id } => route!(GET, Route::game(game_id), Any),
            AddGameMedia { game_id } => route!(POST, Route::game_media(game_id), Token),
            EditGame { game_id } => route!(PUT, Route::game(game_id), Token),
            GetGameTags { game_id } => route!(GET, Route::game_tags(game_id), Any),
            AddGameTags { game_id } => route!(POST, Route::game_tags(game_id), Token),
            DeleteGameTags { game_id } => route!(DELETE, Route::game_tags(game_id), Token),
            GetMods { game_id } => route!(GET, Route::mods(game_id), Any),
            GetMod { game_id, mod_id } => route!(GET, Route::mod_(game_id, mod_id), Any),
            AddMod { game_id } => route!(POST, Route::mods(game_id), Token),
            EditMod { game_id, mod_id } => route!(PUT, Route::mod_(game_id, mod_id), Token),
            DeleteMod { game_id, mod_id } => route!(DELETE, Route::mod_(game_id, mod_id), Token),
            AddModMedia { game_id, mod_id } => {
                route!(POST, Route::mod_media(game_id, mod_id), Token)
            }
            DeleteModMedia { game_id, mod_id } => {
                route!(DELETE, Route::mod_media(game_id, mod_id), Token)
            }
            Subscribe { game_id, mod_id } => {
                route!(POST, Route::mod_subscribe(game_id, mod_id), Token)
            }
            Unsubscribe { game_id, mod_id } => {
                route!(DELETE, Route::mod_subscribe(game_id, mod_id), Token)
            }
            GetAllModEvents { game_id } => route!(GET, Route::mods_events(game_id), Any),
            GetModEvents { game_id, mod_id } => {
                route!(GET, Route::mod_events(game_id, mod_id), Any)
            }
            GetAllModStats { game_id } => route!(GET, Route::mods_stats(game_id), Any),
            GetModStats { game_id, mod_id } => route!(GET, Route::mod_stats(game_id, mod_id), Any),
            GetModTags { game_id, mod_id } => route!(GET, Route::mod_tags(game_id, mod_id), Any),
            AddModTags { game_id, mod_id } => route!(POST, Route::mod_tags(game_id, mod_id), Token),
            DeleteModTags { game_id, mod_id } => {
                route!(DELETE, Route::mod_tags(game_id, mod_id), Token)
            }
            RateMod { game_id, mod_id } => route!(POST, Route::mod_rating(game_id, mod_id), Token),
            GetModMetadata { game_id, mod_id } => {
                route!(GET, Route::mod_metadata(game_id, mod_id), Any)
            }
            AddModMetadata { game_id, mod_id } => {
                route!(POST, Route::mod_metadata(game_id, mod_id), Token)
            }
            DeleteModMetadata { game_id, mod_id } => {
                route!(DELETE, Route::mod_metadata(game_id, mod_id), Token)
            }
            GetModDependencies { game_id, mod_id } => {
                route!(GET, Route::mod_deps(game_id, mod_id), Any)
            }
            AddModDepencencies { game_id, mod_id } => {
                route!(POST, Route::mod_deps(game_id, mod_id), Token)
            }
            DeleteModDependencies { game_id, mod_id } => {
                route!(DELETE, Route::mod_deps(game_id, mod_id), Token)
            }
            GetTeamMembers { game_id, mod_id } => {
                route!(GET, Route::mod_team_members(game_id, mod_id), Any)
            }
            AddTeamMember { game_id, mod_id } => {
                route!(POST, Route::mod_team_members(game_id, mod_id), Token)
            }
            EditTeamMember {
                game_id,
                mod_id,
                member_id,
            } => route!(
                PUT,
                Route::mod_team_member(game_id, mod_id, member_id),
                Token
            ),
            DeleteTeamMember {
                game_id,
                mod_id,
                member_id,
            } => route!(
                DELETE,
                Route::mod_team_member(game_id, mod_id, member_id),
                Token
            ),
            GetModComments { game_id, mod_id } => {
                route!(GET, Route::mod_comments(game_id, mod_id), Any)
            }
            GetModComment {
                game_id,
                mod_id,
                comment_id,
            } => route!(GET, Route::mod_comment(game_id, mod_id, comment_id), Any),
            DeleteModComment {
                game_id,
                mod_id,
                comment_id,
            } => route!(
                DELETE,
                Route::mod_comment(game_id, mod_id, comment_id),
                Token
            ),
            GetFiles { game_id, mod_id } => route!(GET, Route::mod_files(game_id, mod_id), Any),
            GetFile {
                game_id,
                mod_id,
                file_id,
            } => route!(GET, Route::mod_file(game_id, mod_id, file_id), Any),
            AddFile { game_id, mod_id } => route!(POST, Route::mod_files(game_id, mod_id), Token),
            EditFile {
                game_id,
                mod_id,
                file_id,
            } => route!(PUT, Route::mod_file(game_id, mod_id, file_id), Token),
            DeleteFile {
                game_id,
                mod_id,
                file_id,
            } => route!(DELETE, Route::mod_file(game_id, mod_id, file_id), Token),
            AuthorizedUser => route!(GET, Route::user(), Token),
            UserSubscriptions => route!(GET, Route::user_subscriptions(), Token),
            UserEvents => route!(GET, Route::user_events(), Token),
            UserGames => route!(GET, Route::user_games(), Token),
            UserMods => route!(GET, Route::user_mods(), Token),
            UserFiles => route!(GET, Route::user_files(), Token),
            UserRatings => route!(GET, Route::user_ratings(), Token),
            SubmitReport => route!(POST, Route::report(), Token),
        }
    }

    pub fn auth_email_request() -> &'static str {
        "/oauth/emailrequest"
    }

    pub fn auth_email_exchange() -> &'static str {
        "/oauth/emailexchange"
    }

    pub fn auth_steam() -> &'static str {
        "/external/steamauth"
    }

    pub fn auth_gog() -> &'static str {
        "/external/galaxyauth"
    }

    pub fn auth_oculus() -> &'static str {
        "/external/oculusauth"
    }

    pub fn link_account() -> &'static str {
        "/external/link"
    }

    pub fn games() -> &'static str {
        "/games"
    }

    pub fn game(id: u32) -> String {
        format!("/games/{}", id)
    }

    pub fn game_media(id: u32) -> String {
        format!("/games/{}/media", id)
    }

    pub fn game_tags(id: u32) -> String {
        format!("/games/{}/tags", id)
    }

    pub fn mods(game_id: u32) -> String {
        format!("/games/{}/mods", game_id)
    }

    pub fn mods_events(game_id: u32) -> String {
        format!("/games/{}/mods/events", game_id)
    }

    pub fn mods_stats(game_id: u32) -> String {
        format!("/games/{}/mods/stats", game_id)
    }

    pub fn mod_(game_id: u32, mod_id: u32) -> String {
        format!("/games/{}/mods/{}", game_id, mod_id)
    }

    pub fn mod_files(game_id: u32, mod_id: u32) -> String {
        format!("/games/{}/mods/{}/files", game_id, mod_id)
    }

    pub fn mod_file(game_id: u32, mod_id: u32, file_id: u32) -> String {
        format!("/games/{}/mods/{}/files/{}", game_id, mod_id, file_id)
    }

    pub fn mod_media(game_id: u32, mod_id: u32) -> String {
        format!("/games/{}/mods/{}/media", game_id, mod_id)
    }

    pub fn mod_subscribe(game_id: u32, mod_id: u32) -> String {
        format!("/games/{}/mods/{}/subscribe", game_id, mod_id)
    }

    pub fn mod_events(game_id: u32, mod_id: u32) -> String {
        format!("/games/{}/mods/{}/events", game_id, mod_id)
    }

    pub fn mod_stats(game_id: u32, mod_id: u32) -> String {
        format!("/games/{}/mods/{}/stats", game_id, mod_id)
    }

    pub fn mod_tags(game_id: u32, mod_id: u32) -> String {
        format!("/games/{}/mods/{}/tags", game_id, mod_id)
    }

    pub fn mod_rating(game_id: u32, mod_id: u32) -> String {
        format!("/games/{}/mods/{}/ratings", game_id, mod_id)
    }

    pub fn mod_metadata(game_id: u32, mod_id: u32) -> String {
        format!("/games/{}/mods/{}/metadatakvp", game_id, mod_id)
    }

    pub fn mod_deps(game_id: u32, mod_id: u32) -> String {
        format!("/games/{}/mods/{}/dependencies", game_id, mod_id)
    }

    pub fn mod_team_members(game_id: u32, mod_id: u32) -> String {
        format!("/games/{}/mods/{}/team", game_id, mod_id)
    }

    pub fn mod_team_member(game_id: u32, mod_id: u32, member_id: u32) -> String {
        format!("/games/{}/mods/{}/team/{}", game_id, mod_id, member_id)
    }

    pub fn mod_comments(game_id: u32, mod_id: u32) -> String {
        format!("/games/{}/mods/{}/comment", game_id, mod_id)
    }

    pub fn mod_comment(game_id: u32, mod_id: u32, comment_id: u32) -> String {
        format!("/games/{}/mods/{}/comment/{}", game_id, mod_id, comment_id)
    }

    pub fn report() -> &'static str {
        "/report"
    }

    pub fn user() -> &'static str {
        "/me"
    }

    pub fn user_events() -> &'static str {
        "/me/events"
    }

    pub fn user_files() -> &'static str {
        "/me/files"
    }

    pub fn user_games() -> &'static str {
        "/me/games"
    }

    pub fn user_mods() -> &'static str {
        "/me/mods"
    }

    pub fn user_ratings() -> &'static str {
        "/me/ratings"
    }

    pub fn user_subscriptions() -> &'static str {
        "/me/subscribed"
    }
}
