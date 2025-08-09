use std::fmt;

use http::{Method, Uri};

use crate::types::id::{CommentId, FileId, GameId, ModId, UserId};

#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum Route {
    AddFile {
        game_id: GameId,
        mod_id: ModId,
    },
    AddGameMedia {
        game_id: GameId,
    },
    AddGameTags {
        game_id: GameId,
    },
    AddMod {
        game_id: GameId,
    },
    AddModComment {
        game_id: GameId,
        mod_id: ModId,
    },
    AddModDependencies {
        game_id: GameId,
        mod_id: ModId,
    },
    AddModMedia {
        game_id: GameId,
        mod_id: ModId,
    },
    AddModMetadata {
        game_id: GameId,
        mod_id: ModId,
    },
    AddModTags {
        game_id: GameId,
        mod_id: ModId,
    },
    DeleteFile {
        game_id: GameId,
        mod_id: ModId,
        file_id: FileId,
    },
    DeleteGameTags {
        game_id: GameId,
    },
    DeleteMod {
        game_id: GameId,
        mod_id: ModId,
    },
    DeleteModComment {
        game_id: GameId,
        mod_id: ModId,
        comment_id: CommentId,
    },
    DeleteModDependencies {
        game_id: GameId,
        mod_id: ModId,
    },
    DeleteModMedia {
        game_id: GameId,
        mod_id: ModId,
    },
    DeleteModMetadata {
        game_id: GameId,
        mod_id: ModId,
    },
    DeleteModTags {
        game_id: GameId,
        mod_id: ModId,
    },
    EditFile {
        game_id: GameId,
        mod_id: ModId,
        file_id: FileId,
    },
    EditMod {
        game_id: GameId,
        mod_id: ModId,
    },
    EditModComment {
        game_id: GameId,
        mod_id: ModId,
        comment_id: CommentId,
    },
    ExternalAuthDiscord,
    ExternalAuthEpic,
    ExternalAuthGoogle,
    ExternalAuthMeta,
    ExternalAuthOpenID,
    ExternalAuthPSN,
    ExternalAuthSteam,
    ExternalAuthSwitch,
    ExternalAuthXbox,
    GetFile {
        game_id: GameId,
        mod_id: ModId,
        file_id: FileId,
    },
    GetFiles {
        game_id: GameId,
        mod_id: ModId,
    },
    GetGame {
        id: GameId,
        show_hidden_tags: Option<bool>,
    },
    GetGames {
        show_hidden_tags: Option<bool>,
    },
    GetGameStats {
        game_id: GameId,
    },
    GetGameTags {
        game_id: GameId,
        show_hidden_tags: Option<bool>,
    },
    GetMod {
        game_id: GameId,
        mod_id: ModId,
    },
    GetModComment {
        game_id: GameId,
        mod_id: ModId,
        comment_id: CommentId,
    },
    GetModComments {
        game_id: GameId,
        mod_id: ModId,
    },
    GetModDependencies {
        game_id: GameId,
        mod_id: ModId,
        recursive: Option<bool>,
    },
    GetModEvents {
        game_id: GameId,
        mod_id: ModId,
    },
    GetModMetadata {
        game_id: GameId,
        mod_id: ModId,
    },
    GetModStats {
        game_id: GameId,
        mod_id: ModId,
    },
    GetModTags {
        game_id: GameId,
        mod_id: ModId,
    },
    GetModTeamMembers {
        game_id: GameId,
        mod_id: ModId,
    },
    GetMods {
        game_id: GameId,
    },
    GetModsEvents {
        game_id: GameId,
    },
    GetModsStats {
        game_id: GameId,
    },
    ManagePlatformStatus {
        game_id: GameId,
        mod_id: ModId,
        file_id: FileId,
    },
    MuteUser {
        user_id: UserId,
    },
    OAuthEmailRequest,
    OAuthEmailExchange,
    OAuthLogout,
    RateMod {
        game_id: GameId,
        mod_id: ModId,
    },
    RenameGameTags {
        game_id: GameId,
    },
    ReorderModMedia {
        game_id: GameId,
        mod_id: ModId,
    },
    SubmitReport,
    SubscribeToMod {
        game_id: GameId,
        mod_id: ModId,
    },
    Terms,
    UnmuteUser {
        user_id: UserId,
    },
    UnsubscribeFromMod {
        game_id: GameId,
        mod_id: ModId,
    },
    UpdateModCommentKarma {
        game_id: GameId,
        mod_id: ModId,
        comment_id: CommentId,
    },
    UserAuthenticated,
    UserEvents,
    UserFiles,
    UserGames,
    UserMods,
    UserMuted,
    UserRatings,
    UserSubscriptions,
}

pub struct Parts {
    pub method: Method,
    pub path: Path,
    pub token_required: bool,
}

pub struct Path(Route);

impl Route {
    pub const fn method(&self) -> Method {
        match self {
            Self::GetFile { .. }
            | Self::GetFiles { .. }
            | Self::GetGame { .. }
            | Self::GetGames { .. }
            | Self::GetGameStats { .. }
            | Self::GetGameTags { .. }
            | Self::GetMod { .. }
            | Self::GetModComment { .. }
            | Self::GetModComments { .. }
            | Self::GetModDependencies { .. }
            | Self::GetModEvents { .. }
            | Self::GetModMetadata { .. }
            | Self::GetMods { .. }
            | Self::GetModsEvents { .. }
            | Self::GetModsStats { .. }
            | Self::GetModStats { .. }
            | Self::GetModTags { .. }
            | Self::GetModTeamMembers { .. }
            | Self::Terms
            | Self::UserAuthenticated
            | Self::UserEvents
            | Self::UserFiles
            | Self::UserGames
            | Self::UserMods
            | Self::UserRatings
            | Self::UserSubscriptions => Method::GET,
            Self::AddFile { .. }
            | Self::AddGameMedia { .. }
            | Self::AddGameTags { .. }
            | Self::AddMod { .. }
            | Self::AddModComment { .. }
            | Self::AddModDependencies { .. }
            | Self::AddModMedia { .. }
            | Self::AddModMetadata { .. }
            | Self::AddModTags { .. }
            | Self::ExternalAuthDiscord
            | Self::ExternalAuthEpic
            | Self::ExternalAuthGoogle
            | Self::ExternalAuthMeta
            | Self::ExternalAuthOpenID
            | Self::ExternalAuthPSN
            | Self::ExternalAuthSteam
            | Self::ExternalAuthSwitch
            | Self::ExternalAuthXbox
            | Self::ManagePlatformStatus { .. }
            | Self::MuteUser { .. }
            | Self::OAuthEmailRequest
            | Self::OAuthEmailExchange
            | Self::OAuthLogout
            | Self::RateMod { .. }
            | Self::SubmitReport { .. }
            | Self::SubscribeToMod { .. }
            | Self::UpdateModCommentKarma { .. }
            | Self::UserMuted => Method::POST,
            Self::EditMod { .. }
            | Self::EditModComment { .. }
            | Self::EditFile { .. }
            | Self::RenameGameTags { .. }
            | Self::ReorderModMedia { .. } => Method::PUT,
            Self::DeleteFile { .. }
            | Self::DeleteGameTags { .. }
            | Self::DeleteMod { .. }
            | Self::DeleteModComment { .. }
            | Self::DeleteModDependencies { .. }
            | Self::DeleteModMedia { .. }
            | Self::DeleteModMetadata { .. }
            | Self::DeleteModTags { .. }
            | Self::UnmuteUser { .. }
            | Self::UnsubscribeFromMod { .. } => Method::DELETE,
        }
    }

    pub const fn token_required(&self) -> bool {
        match self {
            Self::ExternalAuthDiscord
            | Self::ExternalAuthEpic
            | Self::ExternalAuthGoogle
            | Self::ExternalAuthMeta
            | Self::ExternalAuthOpenID
            | Self::ExternalAuthPSN
            | Self::ExternalAuthSteam
            | Self::ExternalAuthSwitch
            | Self::ExternalAuthXbox
            | Self::GetFile { .. }
            | Self::GetFiles { .. }
            | Self::GetGame { .. }
            | Self::GetGames { .. }
            | Self::GetGameStats { .. }
            | Self::GetGameTags { .. }
            | Self::GetMod { .. }
            | Self::GetModComment { .. }
            | Self::GetModComments { .. }
            | Self::GetModDependencies { .. }
            | Self::GetModEvents { .. }
            | Self::GetModMetadata { .. }
            | Self::GetMods { .. }
            | Self::GetModsEvents { .. }
            | Self::GetModsStats { .. }
            | Self::GetModStats { .. }
            | Self::GetModTags { .. }
            | Self::GetModTeamMembers { .. }
            | Self::OAuthEmailRequest
            | Self::OAuthEmailExchange
            | Self::Terms => false,
            Self::AddFile { .. }
            | Self::AddGameMedia { .. }
            | Self::AddGameTags { .. }
            | Self::AddMod { .. }
            | Self::AddModComment { .. }
            | Self::AddModDependencies { .. }
            | Self::AddModMedia { .. }
            | Self::AddModMetadata { .. }
            | Self::AddModTags { .. }
            | Self::DeleteFile { .. }
            | Self::DeleteGameTags { .. }
            | Self::DeleteMod { .. }
            | Self::DeleteModComment { .. }
            | Self::DeleteModDependencies { .. }
            | Self::DeleteModMedia { .. }
            | Self::DeleteModMetadata { .. }
            | Self::DeleteModTags { .. }
            | Self::EditFile { .. }
            | Self::EditMod { .. }
            | Self::EditModComment { .. }
            | Self::ManagePlatformStatus { .. }
            | Self::MuteUser { .. }
            | Self::OAuthLogout
            | Self::RateMod { .. }
            | Self::RenameGameTags { .. }
            | Self::ReorderModMedia { .. }
            | Self::SubmitReport { .. }
            | Self::SubscribeToMod { .. }
            | Self::UnmuteUser { .. }
            | Self::UnsubscribeFromMod { .. }
            | Self::UpdateModCommentKarma { .. }
            | Self::UserAuthenticated
            | Self::UserEvents
            | Self::UserFiles
            | Self::UserGames
            | Self::UserMods
            | Self::UserMuted
            | Self::UserRatings
            | Self::UserSubscriptions => true,
        }
    }

    pub fn into_parts(self) -> Parts {
        Parts {
            method: self.method(),
            path: Path(self),
            token_required: self.token_required(),
        }
    }
}

macro_rules! path {
    ($($pieces:tt)*) => { internal_path!(@start $($pieces)*)};
}

macro_rules! internal_path {
    (@start $f:ident; $first:tt $(, $tail:tt)*) => ({
        internal_path!(@munch $f; [$first] [$(, $tail)*])
    });
    (@munch $f:ident; [$curr:tt] [, $next:tt $(, $tail:tt)*]) => ({
        internal_path!(@segment $f; $curr);
        internal_path!(@munch $f; [$next] [$(, $tail)*])
    });
    (@munch $f:ident; [$curr:tt] []) => ({
        internal_path!(@segment $f; $curr);
        Ok(())
    });
    (@segment $f:ident; $s:literal) => (
        $f.write_str($s)?;
    );
    (@segment $f:ident; $v:ident) => (
        fmt::Display::fmt($v, $f)?;
    );
    (@segment $f:ident; $block:block) => (
        $block
    );
}

impl TryFrom<Path> for Uri {
    type Error = <Uri as TryFrom<String>>::Error;

    fn try_from(value: Path) -> Result<Self, Self::Error> {
        Self::try_from(value.to_string())
    }
}

impl fmt::Display for Path {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Route::AddFile { game_id, mod_id } | Route::GetFiles { game_id, mod_id } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/files")
            }
            Route::AddGameMedia { game_id } => {
                path!(f; "/games/", game_id, "/media")
            }
            Route::AddGameTags { game_id } | Route::DeleteGameTags { game_id } => {
                path!(f; "/games/", game_id, "/tags")
            }
            Route::AddMod { game_id } | Route::GetMods { game_id } => {
                path!(f; "/games/", game_id, "/mods")
            }
            Route::AddModComment { game_id, mod_id }
            | Route::GetModComments { game_id, mod_id } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/comments")
            }
            Route::AddModDependencies { game_id, mod_id }
            | Route::DeleteModDependencies { game_id, mod_id } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/dependencies")
            }
            Route::AddModMedia { game_id, mod_id } | Route::DeleteModMedia { game_id, mod_id } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/media")
            }
            Route::AddModMetadata { game_id, mod_id }
            | Route::DeleteModMetadata { game_id, mod_id }
            | Route::GetModMetadata { game_id, mod_id } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/metadatakvp")
            }
            Route::AddModTags { game_id, mod_id }
            | Route::DeleteModTags { game_id, mod_id }
            | Route::GetModTags { game_id, mod_id } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/tags")
            }
            Route::DeleteFile {
                game_id,
                mod_id,
                file_id,
            }
            | Route::EditFile {
                game_id,
                mod_id,
                file_id,
            }
            | Route::GetFile {
                game_id,
                mod_id,
                file_id,
            } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/files/", file_id)
            }
            Route::DeleteMod { game_id, mod_id }
            | Route::EditMod { game_id, mod_id }
            | Route::GetMod { game_id, mod_id } => {
                path!(f; "/games/", game_id, "/mods/", mod_id)
            }
            Route::DeleteModComment {
                game_id,
                mod_id,
                comment_id,
            }
            | Route::EditModComment {
                game_id,
                mod_id,
                comment_id,
            }
            | Route::GetModComment {
                game_id,
                mod_id,
                comment_id,
            } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/comments/", comment_id)
            }
            Route::ExternalAuthDiscord => f.write_str("/external/discordauth"),
            Route::ExternalAuthEpic => f.write_str("/external/epicgamesauth"),
            Route::ExternalAuthGoogle => f.write_str("/external/googleauth"),
            Route::ExternalAuthMeta => f.write_str("/external/oculusauth"),
            Route::ExternalAuthOpenID => f.write_str("/external/openidauth"),
            Route::ExternalAuthPSN => f.write_str("/external/psnauth"),
            Route::ExternalAuthSteam => f.write_str("/external/steamauth"),
            Route::ExternalAuthSwitch => f.write_str("/external/switchauth"),
            Route::ExternalAuthXbox => f.write_str("/external/xboxauth"),
            Route::GetGame {
                id,
                show_hidden_tags,
            } => {
                path!(f; "/games/", id, {
                    if let Some(show_hidden_tags) = show_hidden_tags {
                        f.write_str("?show_hidden_tags=")?;
                        fmt::Display::fmt(show_hidden_tags, f)?;
                    }
                })
            }
            Route::GetGames { show_hidden_tags } => {
                path!(f; "/games", {
                    if let Some(show_hidden_tags) = show_hidden_tags {
                        f.write_str("?show_hidden_tags=")?;
                        fmt::Display::fmt(show_hidden_tags, f)?;
                    }
                })
            }
            Route::GetGameStats { game_id } => {
                path!(f; "/games/", game_id, "/stats")
            }
            Route::GetGameTags {
                game_id,
                show_hidden_tags,
            } => {
                path!(f; "/games/", game_id, "/tags", {
                    if let Some(show_hidden_tags) = show_hidden_tags {
                        f.write_str("?show_hidden_tags=")?;
                        fmt::Display::fmt(show_hidden_tags, f)?;
                    }
                })
            }
            Route::GetModDependencies {
                game_id,
                mod_id,
                recursive,
            } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/dependencies", {
                    if let Some(recursive) = recursive {
                        f.write_str("?recursive=")?;
                        fmt::Display::fmt(recursive, f)?;
                    }
                })
            }
            Route::GetModEvents { game_id, mod_id } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/events")
            }
            Route::GetModTeamMembers { game_id, mod_id } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/team")
            }
            Route::GetModsEvents { game_id } => {
                path!(f; "/games/", game_id, "/mods/events")
            }
            Route::GetModsStats { game_id } => {
                path!(f; "/games/", game_id, "/mods/stats")
            }
            Route::GetModStats { game_id, mod_id } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/stats")
            }
            Route::ManagePlatformStatus {
                game_id,
                mod_id,
                file_id,
            } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/files/", file_id, "/platforms")
            }
            Route::MuteUser { user_id } | Route::UnmuteUser { user_id } => {
                path!(f; "/users/", user_id, "/mute")
            }
            Route::OAuthEmailRequest => f.write_str("/oauth/emailrequest"),
            Route::OAuthEmailExchange => f.write_str("/oauth/emailexchange"),
            Route::OAuthLogout => f.write_str("/oauth/logout"),
            Route::RateMod { game_id, mod_id } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/ratings")
            }
            Route::RenameGameTags { game_id } => {
                path!(f; "/games/", game_id, "/tags/rename")
            }
            Route::ReorderModMedia { game_id, mod_id } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/media/reorder")
            }
            Route::SubscribeToMod { game_id, mod_id }
            | Route::UnsubscribeFromMod { game_id, mod_id } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/subscribe")
            }
            Route::SubmitReport => f.write_str("/report"),
            Route::Terms => f.write_str("/authenticate/terms"),
            Route::UpdateModCommentKarma {
                game_id,
                mod_id,
                comment_id,
            } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/comments/", comment_id, "/karma")
            }
            Route::UserAuthenticated => f.write_str("/me"),
            Route::UserEvents => f.write_str("/me/events"),
            Route::UserFiles => f.write_str("/me/files"),
            Route::UserGames => f.write_str("/me/games"),
            Route::UserMods => f.write_str("/me/mods"),
            Route::UserMuted => f.write_str("/me/users/muted"),
            Route::UserRatings => f.write_str("/me/ratings"),
            Route::UserSubscriptions => f.write_str("/me/subscribed"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GAME_ID: GameId = GameId::new(1);
    const MOD_ID: ModId = ModId::new(2);
    const FILE_ID: FileId = FileId::new(3);
    const COMMENT_ID: CommentId = CommentId::new(4);
    const USER_ID: UserId = UserId::new(5);

    #[test]
    fn add_file() {
        let route = Route::AddFile {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2/files");
    }

    #[test]
    fn add_game_media() {
        let route = Route::AddGameMedia { game_id: GAME_ID };

        assert_eq!(Path(route).to_string(), "/games/1/media");
    }

    #[test]
    fn add_game_tags() {
        let route = Route::AddGameTags { game_id: GAME_ID };

        assert_eq!(Path(route).to_string(), "/games/1/tags");
    }

    #[test]
    fn add_mod() {
        let route = Route::AddMod { game_id: GAME_ID };

        assert_eq!(Path(route).to_string(), "/games/1/mods");
    }

    #[test]
    fn add_mod_comment() {
        let route = Route::AddModComment {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2/comments");
    }

    #[test]
    fn add_mod_dependencies() {
        let route = Route::AddModDependencies {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2/dependencies");
    }

    #[test]
    fn add_mod_media() {
        let route = Route::AddModMedia {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2/media");
    }

    #[test]
    fn add_mod_metadata() {
        let route = Route::AddModMetadata {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2/metadatakvp");
    }

    #[test]
    fn add_mod_tags() {
        let route = Route::AddModTags {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2/tags");
    }

    #[test]
    fn delete_file() {
        let route = Route::DeleteFile {
            game_id: GAME_ID,
            mod_id: MOD_ID,
            file_id: FILE_ID,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2/files/3");
    }

    #[test]
    fn delete_game_tags() {
        let route = Route::DeleteGameTags { game_id: GAME_ID };

        assert_eq!(Path(route).to_string(), "/games/1/tags");
    }

    #[test]
    fn delete_mod() {
        let route = Route::DeleteMod {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2");
    }

    #[test]
    fn delete_mod_comment() {
        let route = Route::DeleteModComment {
            game_id: GAME_ID,
            mod_id: MOD_ID,
            comment_id: COMMENT_ID,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2/comments/4");
    }

    #[test]
    fn delete_mod_dependencies() {
        let route = Route::DeleteModDependencies {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2/dependencies");
    }

    #[test]
    fn delete_mod_media() {
        let route = Route::DeleteModMedia {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2/media");
    }

    #[test]
    fn delete_mod_metadata() {
        let route = Route::DeleteModMetadata {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2/metadatakvp");
    }

    #[test]
    fn delete_mod_tags() {
        let route = Route::DeleteModTags {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2/tags");
    }

    #[test]
    fn edit_file() {
        let route = Route::EditFile {
            game_id: GAME_ID,
            mod_id: MOD_ID,
            file_id: FILE_ID,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2/files/3");
    }

    #[test]
    fn edit_mod() {
        let route = Route::EditMod {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2");
    }

    #[test]
    fn edit_mod_comment() {
        let route = Route::EditModComment {
            game_id: GAME_ID,
            mod_id: MOD_ID,
            comment_id: COMMENT_ID,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2/comments/4");
    }

    #[test]
    fn external_auth_discord() {
        let route = Route::ExternalAuthDiscord;

        assert_eq!(Path(route).to_string(), "/external/discordauth");
    }

    #[test]
    fn external_auth_epic() {
        let route = Route::ExternalAuthEpic;

        assert_eq!(Path(route).to_string(), "/external/epicgamesauth");
    }

    #[test]
    fn external_auth_google() {
        let route = Route::ExternalAuthGoogle;

        assert_eq!(Path(route).to_string(), "/external/googleauth");
    }

    #[test]
    fn external_auth_meta() {
        let route = Route::ExternalAuthMeta;

        assert_eq!(Path(route).to_string(), "/external/oculusauth");
    }

    #[test]
    fn external_auth_openid() {
        let route = Route::ExternalAuthOpenID;

        assert_eq!(Path(route).to_string(), "/external/openidauth");
    }

    #[test]
    fn external_auth_psn() {
        let route = Route::ExternalAuthPSN;

        assert_eq!(Path(route).to_string(), "/external/psnauth");
    }

    #[test]
    fn external_auth_steam() {
        let route = Route::ExternalAuthSteam;

        assert_eq!(Path(route).to_string(), "/external/steamauth");
    }

    #[test]
    fn external_auth_switch() {
        let route = Route::ExternalAuthSwitch;

        assert_eq!(Path(route).to_string(), "/external/switchauth");
    }

    #[test]
    fn external_auth_xbox() {
        let route = Route::ExternalAuthXbox;

        assert_eq!(Path(route).to_string(), "/external/xboxauth");
    }

    #[test]
    fn get_file() {
        let route = Route::GetFile {
            game_id: GAME_ID,
            mod_id: MOD_ID,
            file_id: FILE_ID,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2/files/3");
    }

    #[test]
    fn get_files() {
        let route = Route::GetFiles {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2/files");
    }

    #[test]
    fn get_game() {
        let route = Route::GetGame {
            id: GAME_ID,
            show_hidden_tags: None,
        };

        assert_eq!(Path(route).to_string(), "/games/1");

        let route = Route::GetGame {
            id: GAME_ID,
            show_hidden_tags: Some(true),
        };

        assert_eq!(Path(route).to_string(), "/games/1?show_hidden_tags=true");
    }

    #[test]
    fn get_games() {
        let route = Route::GetGames {
            show_hidden_tags: None,
        };

        assert_eq!(Path(route).to_string(), "/games");

        let route = Route::GetGames {
            show_hidden_tags: Some(true),
        };

        assert_eq!(Path(route).to_string(), "/games?show_hidden_tags=true");
    }

    #[test]
    fn get_game_stats() {
        let route = Route::GetGameStats { game_id: GAME_ID };

        assert_eq!(Path(route).to_string(), "/games/1/stats");
    }

    #[test]
    fn get_game_tags() {
        let route = Route::GetGameTags {
            game_id: GAME_ID,
            show_hidden_tags: None,
        };

        assert_eq!(Path(route).to_string(), "/games/1/tags");

        let route = Route::GetGameTags {
            game_id: GAME_ID,
            show_hidden_tags: Some(true),
        };

        assert_eq!(
            Path(route).to_string(),
            "/games/1/tags?show_hidden_tags=true"
        );
    }

    #[test]
    fn get_mod() {
        let route = Route::GetMod {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2");
    }

    #[test]
    fn get_mod_comment() {
        let route = Route::GetModComment {
            game_id: GAME_ID,
            mod_id: MOD_ID,
            comment_id: COMMENT_ID,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2/comments/4");
    }

    #[test]
    fn get_mod_comments() {
        let route = Route::GetModComments {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2/comments");
    }

    #[test]
    fn get_mod_dependencies() {
        let route = Route::GetModDependencies {
            game_id: GAME_ID,
            mod_id: MOD_ID,
            recursive: None,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2/dependencies");

        let route = Route::GetModDependencies {
            game_id: GAME_ID,
            mod_id: MOD_ID,
            recursive: Some(true),
        };

        assert_eq!(
            Path(route).to_string(),
            "/games/1/mods/2/dependencies?recursive=true"
        );
    }

    #[test]
    fn get_mod_events() {
        let route = Route::GetModEvents {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2/events");
    }

    #[test]
    fn get_mod_metadata() {
        let route = Route::GetModMetadata {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2/metadatakvp");
    }

    #[test]
    fn get_mod_stats() {
        let route = Route::GetModStats {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2/stats");
    }

    #[test]
    fn get_mod_tags() {
        let route = Route::GetModTags {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2/tags");
    }

    #[test]
    fn get_mod_team_members() {
        let route = Route::GetModTeamMembers {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2/team");
    }

    #[test]
    fn get_mods() {
        let route = Route::GetMods { game_id: GAME_ID };

        assert_eq!(Path(route).to_string(), "/games/1/mods");
    }

    #[test]
    fn get_mods_events() {
        let route = Route::GetModsEvents { game_id: GAME_ID };

        assert_eq!(Path(route).to_string(), "/games/1/mods/events");
    }

    #[test]
    fn get_mods_stats() {
        let route = Route::GetModsStats { game_id: GAME_ID };

        assert_eq!(Path(route).to_string(), "/games/1/mods/stats");
    }

    #[test]
    fn manage_platform_status() {
        let route = Route::ManagePlatformStatus {
            game_id: GAME_ID,
            mod_id: MOD_ID,
            file_id: FILE_ID,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2/files/3/platforms");
    }

    #[test]
    fn mute_user() {
        let route = Route::MuteUser { user_id: USER_ID };

        assert_eq!(Path(route).to_string(), "/users/5/mute");
    }

    #[test]
    fn oauth_email_request() {
        let route = Route::OAuthEmailRequest;

        assert_eq!(Path(route).to_string(), "/oauth/emailrequest");
    }

    #[test]
    fn oauth_email_exchange() {
        let route = Route::OAuthEmailExchange;

        assert_eq!(Path(route).to_string(), "/oauth/emailexchange");
    }

    #[test]
    fn oauth_logout() {
        let route = Route::OAuthLogout;

        assert_eq!(Path(route).to_string(), "/oauth/logout");
    }

    #[test]
    fn rate_mod() {
        let route = Route::RateMod {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2/ratings");
    }

    #[test]
    fn rename_game_tags() {
        let route = Route::RenameGameTags { game_id: GAME_ID };

        assert_eq!(Path(route).to_string(), "/games/1/tags/rename");
    }

    #[test]
    fn reorder_mod_media() {
        let route = Route::ReorderModMedia {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2/media/reorder");
    }

    #[test]
    fn submit_report() {
        let route = Route::SubmitReport;

        assert_eq!(Path(route).to_string(), "/report");
    }

    #[test]
    fn subscribe_to_mod() {
        let route = Route::SubscribeToMod {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2/subscribe");
    }

    #[test]
    fn update_mod_comment_karma() {
        let route = Route::UpdateModCommentKarma {
            game_id: GAME_ID,
            mod_id: MOD_ID,
            comment_id: COMMENT_ID,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2/comments/4/karma");
    }

    #[test]
    fn terms() {
        let route = Route::Terms;

        assert_eq!(Path(route).to_string(), "/authenticate/terms");
    }

    #[test]
    fn unmute_user() {
        let route = Route::UnmuteUser { user_id: USER_ID };

        assert_eq!(Path(route).to_string(), "/users/5/mute");
    }

    #[test]
    fn unsubscribe_from_mod() {
        let route = Route::UnsubscribeFromMod {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(Path(route).to_string(), "/games/1/mods/2/subscribe");
    }

    #[test]
    fn user_authenticated() {
        let route = Route::UserAuthenticated;

        assert_eq!(Path(route).to_string(), "/me");
    }

    #[test]
    fn user_events() {
        let route = Route::UserEvents;

        assert_eq!(Path(route).to_string(), "/me/events");
    }

    #[test]
    fn user_files() {
        let route = Route::UserFiles;

        assert_eq!(Path(route).to_string(), "/me/files");
    }

    #[test]
    fn user_games() {
        let route = Route::UserGames;

        assert_eq!(Path(route).to_string(), "/me/games");
    }

    #[test]
    fn user_mods() {
        let route = Route::UserMods;

        assert_eq!(Path(route).to_string(), "/me/mods");
    }

    #[test]
    fn user_muted() {
        let route = Route::UserMuted;

        assert_eq!(Path(route).to_string(), "/me/users/muted");
    }

    #[test]
    fn user_ratings() {
        let route = Route::UserRatings;

        assert_eq!(Path(route).to_string(), "/me/ratings");
    }

    #[test]
    fn user_subscriptions() {
        let route = Route::UserSubscriptions;

        assert_eq!(Path(route).to_string(), "/me/subscribed");
    }
}
