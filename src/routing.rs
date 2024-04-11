use std::fmt;

use http::Method;

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
    AddModCommentKarma {
        game_id: GameId,
        mod_id: ModId,
        comment_id: CommentId,
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
    #[allow(dead_code)]
    ExternalAuthEpic,
    ExternalAuthGoogle,
    ExternalAuthMeta,
    #[allow(dead_code)]
    ExternalAuthOpenID,
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
    pub path: String,
    pub token_required: bool,
}

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
            | Self::AddModCommentKarma { .. }
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
            | Self::AddModCommentKarma { .. }
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
            path: self.to_string(),
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
    (@segment $f:ident; $s:literal) => ({
        $f.write_str($s)?;
    });
    (@segment $f:ident; $v:ident) => ({
        fmt::Display::fmt($v, $f)?;
    });
}

impl fmt::Display for Route {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AddFile { game_id, mod_id } | Self::GetFiles { game_id, mod_id } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/files")
            }
            Self::AddGameMedia { game_id } => {
                path!(f; "/games/", game_id, "/media")
            }
            Self::AddGameTags { game_id }
            | Self::DeleteGameTags { game_id }
            | Self::GetGameTags { game_id } => {
                path!(f; "/games/", game_id, "/tags")
            }
            Self::AddMod { game_id } | Self::GetMods { game_id } => {
                path!(f; "/games/", game_id, "/mods")
            }
            Self::AddModComment { game_id, mod_id } | Self::GetModComments { game_id, mod_id } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/comments")
            }
            Self::AddModCommentKarma {
                game_id,
                mod_id,
                comment_id,
            } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/comments/", comment_id, "/karma")
            }
            Self::AddModDependencies { game_id, mod_id }
            | Self::DeleteModDependencies { game_id, mod_id }
            | Self::GetModDependencies { game_id, mod_id } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/dependencies")
            }
            Self::AddModMedia { game_id, mod_id } | Self::DeleteModMedia { game_id, mod_id } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/media")
            }
            Self::AddModMetadata { game_id, mod_id }
            | Self::DeleteModMetadata { game_id, mod_id }
            | Self::GetModMetadata { game_id, mod_id } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/metadatakvp")
            }
            Self::AddModTags { game_id, mod_id }
            | Self::DeleteModTags { game_id, mod_id }
            | Self::GetModTags { game_id, mod_id } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/tags")
            }
            Self::DeleteFile {
                game_id,
                mod_id,
                file_id,
            }
            | Self::EditFile {
                game_id,
                mod_id,
                file_id,
            }
            | Self::GetFile {
                game_id,
                mod_id,
                file_id,
            } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/files/", file_id)
            }
            Self::DeleteMod { game_id, mod_id }
            | Self::EditMod { game_id, mod_id }
            | Self::GetMod { game_id, mod_id } => {
                path!(f; "/games/", game_id, "/mods/", mod_id)
            }
            Self::DeleteModComment {
                game_id,
                mod_id,
                comment_id,
            }
            | Self::EditModComment {
                game_id,
                mod_id,
                comment_id,
            }
            | Self::GetModComment {
                game_id,
                mod_id,
                comment_id,
            } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/comments/", comment_id)
            }
            Self::ExternalAuthDiscord => f.write_str("/external/discordauth"),
            Self::ExternalAuthEpic => f.write_str("/external/epicgamesauth"),
            Self::ExternalAuthGoogle => f.write_str("/external/googleauth"),
            Self::ExternalAuthMeta => f.write_str("/external/oculusauth"),
            Self::ExternalAuthOpenID => f.write_str("/external/openidauth"),
            Self::ExternalAuthPSN => f.write_str("/external/psnauth"),
            Self::ExternalAuthSteam => f.write_str("/external/steamauth"),
            Self::ExternalAuthSwitch => f.write_str("/external/switchauth"),
            Self::ExternalAuthXbox => f.write_str("/external/xboxauth"),
            Self::GetGame {
                id,
                show_hidden_tags,
            } => {
                f.write_str("/games/")?;
                fmt::Display::fmt(id, f)?;
                if let Some(show_hidden_tags) = show_hidden_tags {
                    f.write_str("?show_hidden_tags=")?;
                    fmt::Display::fmt(show_hidden_tags, f)?;
                }
                Ok(())
            }
            Self::GetGames { show_hidden_tags } => {
                f.write_str("/games")?;
                if let Some(show_hidden_tags) = show_hidden_tags {
                    f.write_str("?show_hidden_tags=")?;
                    fmt::Display::fmt(show_hidden_tags, f)?;
                }
                Ok(())
            }
            Self::GetGameStats { game_id } => {
                path!(f; "/games/", game_id, "/stats")
            }
            Self::GetModEvents { game_id, mod_id } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/events")
            }
            Self::GetModTeamMembers { game_id, mod_id } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/team")
            }
            Self::GetModsEvents { game_id } => {
                path!(f; "/games/", game_id, "/mods/events")
            }
            Self::GetModsStats { game_id } => {
                path!(f; "/games/", game_id, "/mods/stats")
            }
            Self::GetModStats { game_id, mod_id } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/stats")
            }
            Self::ManagePlatformStatus {
                game_id,
                mod_id,
                file_id,
            } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/files/", file_id, "/platforms")
            }
            Self::MuteUser { user_id } | Self::UnmuteUser { user_id } => {
                path!(f; "/users/", user_id, "/mute")
            }
            Self::OAuthEmailRequest => f.write_str("/oauth/emailrequest"),
            Self::OAuthEmailExchange => f.write_str("/oauth/emailexchange"),
            Self::OAuthLogout => f.write_str("/oauth/logout"),
            Self::RateMod { game_id, mod_id } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/ratings")
            }
            Self::RenameGameTags { game_id } => {
                path!(f; "/games/", game_id, "/tags/rename")
            }
            Self::ReorderModMedia { game_id, mod_id } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/media/reorder")
            }
            Self::SubscribeToMod { game_id, mod_id }
            | Self::UnsubscribeFromMod { game_id, mod_id } => {
                path!(f; "/games/", game_id, "/mods/", mod_id, "/subscribe")
            }
            Self::SubmitReport => f.write_str("/report"),
            Self::Terms => f.write_str("/authenticate/terms"),
            Self::UserAuthenticated => f.write_str("/me"),
            Self::UserEvents => f.write_str("/me/events"),
            Self::UserFiles => f.write_str("/me/files"),
            Self::UserGames => f.write_str("/me/games"),
            Self::UserMods => f.write_str("/me/mods"),
            Self::UserMuted => f.write_str("/me/users/muted"),
            Self::UserRatings => f.write_str("/me/ratings"),
            Self::UserSubscriptions => f.write_str("/me/subscribed"),
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

        assert_eq!(route.to_string(), "/games/1/mods/2/files");
    }

    #[test]
    fn add_game_media() {
        let route = Route::AddGameMedia { game_id: GAME_ID };

        assert_eq!(route.to_string(), "/games/1/media");
    }

    #[test]
    fn add_game_tags() {
        let route = Route::AddGameTags { game_id: GAME_ID };

        assert_eq!(route.to_string(), "/games/1/tags");
    }

    #[test]
    fn add_mod() {
        let route = Route::AddMod { game_id: GAME_ID };

        assert_eq!(route.to_string(), "/games/1/mods");
    }

    #[test]
    fn add_mod_comment() {
        let route = Route::AddModComment {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(route.to_string(), "/games/1/mods/2/comments");
    }

    #[test]
    fn add_mod_comment_karma() {
        let route = Route::AddModCommentKarma {
            game_id: GAME_ID,
            mod_id: MOD_ID,
            comment_id: COMMENT_ID,
        };

        assert_eq!(route.to_string(), "/games/1/mods/2/comments/4/karma");
    }

    #[test]
    fn add_mod_dependencies() {
        let route = Route::AddModDependencies {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(route.to_string(), "/games/1/mods/2/dependencies");
    }

    #[test]
    fn add_mod_media() {
        let route = Route::AddModMedia {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(route.to_string(), "/games/1/mods/2/media");
    }

    #[test]
    fn add_mod_metadata() {
        let route = Route::AddModMetadata {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(route.to_string(), "/games/1/mods/2/metadatakvp");
    }

    #[test]
    fn add_mod_tags() {
        let route = Route::AddModTags {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(route.to_string(), "/games/1/mods/2/tags");
    }

    #[test]
    fn delete_file() {
        let route = Route::DeleteFile {
            game_id: GAME_ID,
            mod_id: MOD_ID,
            file_id: FILE_ID,
        };

        assert_eq!(route.to_string(), "/games/1/mods/2/files/3");
    }

    #[test]
    fn delete_game_tags() {
        let route = Route::DeleteGameTags { game_id: GAME_ID };

        assert_eq!(route.to_string(), "/games/1/tags");
    }

    #[test]
    fn delete_mod() {
        let route = Route::DeleteMod {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(route.to_string(), "/games/1/mods/2");
    }

    #[test]
    fn delete_mod_comment() {
        let route = Route::DeleteModComment {
            game_id: GAME_ID,
            mod_id: MOD_ID,
            comment_id: COMMENT_ID,
        };

        assert_eq!(route.to_string(), "/games/1/mods/2/comments/4");
    }

    #[test]
    fn delete_mod_dependencies() {
        let route = Route::DeleteModDependencies {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(route.to_string(), "/games/1/mods/2/dependencies");
    }

    #[test]
    fn delete_mod_media() {
        let route = Route::DeleteModMedia {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(route.to_string(), "/games/1/mods/2/media");
    }

    #[test]
    fn delete_mod_metadata() {
        let route = Route::DeleteModMetadata {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(route.to_string(), "/games/1/mods/2/metadatakvp");
    }

    #[test]
    fn delete_mod_tags() {
        let route = Route::DeleteModTags {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(route.to_string(), "/games/1/mods/2/tags");
    }

    #[test]
    fn edit_file() {
        let route = Route::EditFile {
            game_id: GAME_ID,
            mod_id: MOD_ID,
            file_id: FILE_ID,
        };

        assert_eq!(route.to_string(), "/games/1/mods/2/files/3");
    }

    #[test]
    fn edit_mod() {
        let route = Route::EditMod {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(route.to_string(), "/games/1/mods/2");
    }

    #[test]
    fn edit_mod_comment() {
        let route = Route::EditModComment {
            game_id: GAME_ID,
            mod_id: MOD_ID,
            comment_id: COMMENT_ID,
        };

        assert_eq!(route.to_string(), "/games/1/mods/2/comments/4");
    }

    #[test]
    fn external_auth_discord() {
        let route = Route::ExternalAuthDiscord;

        assert_eq!(route.to_string(), "/external/discordauth");
    }

    #[test]
    fn external_auth_epic() {
        let route = Route::ExternalAuthEpic;

        assert_eq!(route.to_string(), "/external/epicgamesauth");
    }

    #[test]
    fn external_auth_google() {
        let route = Route::ExternalAuthGoogle;

        assert_eq!(route.to_string(), "/external/googleauth");
    }

    #[test]
    fn external_auth_meta() {
        let route = Route::ExternalAuthMeta;

        assert_eq!(route.to_string(), "/external/oculusauth");
    }

    #[test]
    fn external_auth_openid() {
        let route = Route::ExternalAuthOpenID;

        assert_eq!(route.to_string(), "/external/openidauth");
    }

    #[test]
    fn external_auth_psn() {
        let route = Route::ExternalAuthPSN;

        assert_eq!(route.to_string(), "/external/psnauth");
    }

    #[test]
    fn external_auth_steam() {
        let route = Route::ExternalAuthSteam;

        assert_eq!(route.to_string(), "/external/steamauth");
    }

    #[test]
    fn external_auth_switch() {
        let route = Route::ExternalAuthSwitch;

        assert_eq!(route.to_string(), "/external/switchauth");
    }

    #[test]
    fn external_auth_xbox() {
        let route = Route::ExternalAuthXbox;

        assert_eq!(route.to_string(), "/external/xboxauth");
    }

    #[test]
    fn get_file() {
        let route = Route::GetFile {
            game_id: GAME_ID,
            mod_id: MOD_ID,
            file_id: FILE_ID,
        };

        assert_eq!(route.to_string(), "/games/1/mods/2/files/3");
    }

    #[test]
    fn get_files() {
        let route = Route::GetFiles {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(route.to_string(), "/games/1/mods/2/files");
    }

    #[test]
    fn get_game() {
        let route = Route::GetGame {
            id: GAME_ID,
            show_hidden_tags: None,
        };

        assert_eq!(route.to_string(), "/games/1");

        let route = Route::GetGame {
            id: GAME_ID,
            show_hidden_tags: Some(true),
        };

        assert_eq!(route.to_string(), "/games/1?show_hidden_tags=true");
    }

    #[test]
    fn get_games() {
        let route = Route::GetGames {
            show_hidden_tags: None,
        };

        assert_eq!(route.to_string(), "/games");

        let route = Route::GetGames {
            show_hidden_tags: Some(true),
        };

        assert_eq!(route.to_string(), "/games?show_hidden_tags=true");
    }

    #[test]
    fn get_game_stats() {
        let route = Route::GetGameStats { game_id: GAME_ID };

        assert_eq!(route.to_string(), "/games/1/stats");
    }

    #[test]
    fn get_game_tags() {
        let route = Route::GetGameTags { game_id: GAME_ID };

        assert_eq!(route.to_string(), "/games/1/tags");
    }

    #[test]
    fn get_mod() {
        let route = Route::GetMod {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(route.to_string(), "/games/1/mods/2");
    }

    #[test]
    fn get_mod_comment() {
        let route = Route::GetModComment {
            game_id: GAME_ID,
            mod_id: MOD_ID,
            comment_id: COMMENT_ID,
        };

        assert_eq!(route.to_string(), "/games/1/mods/2/comments/4");
    }

    #[test]
    fn get_mod_comments() {
        let route = Route::GetModComments {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(route.to_string(), "/games/1/mods/2/comments");
    }

    #[test]
    fn get_mod_dependencies() {
        let route = Route::GetModDependencies {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(route.to_string(), "/games/1/mods/2/dependencies");
    }

    #[test]
    fn get_mod_events() {
        let route = Route::GetModEvents {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(route.to_string(), "/games/1/mods/2/events");
    }

    #[test]
    fn get_mod_metadata() {
        let route = Route::GetModMetadata {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(route.to_string(), "/games/1/mods/2/metadatakvp");
    }

    #[test]
    fn get_mod_stats() {
        let route = Route::GetModStats {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(route.to_string(), "/games/1/mods/2/stats");
    }

    #[test]
    fn get_mod_tags() {
        let route = Route::GetModTags {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(route.to_string(), "/games/1/mods/2/tags");
    }

    #[test]
    fn get_mod_team_members() {
        let route = Route::GetModTeamMembers {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(route.to_string(), "/games/1/mods/2/team");
    }

    #[test]
    fn get_mods() {
        let route = Route::GetMods { game_id: GAME_ID };

        assert_eq!(route.to_string(), "/games/1/mods");
    }

    #[test]
    fn get_mods_events() {
        let route = Route::GetModsEvents { game_id: GAME_ID };

        assert_eq!(route.to_string(), "/games/1/mods/events");
    }

    #[test]
    fn get_mods_stats() {
        let route = Route::GetModsStats { game_id: GAME_ID };

        assert_eq!(route.to_string(), "/games/1/mods/stats");
    }

    #[test]
    fn manage_platform_status() {
        let route = Route::ManagePlatformStatus {
            game_id: GAME_ID,
            mod_id: MOD_ID,
            file_id: FILE_ID,
        };

        assert_eq!(route.to_string(), "/games/1/mods/2/files/3/platforms");
    }

    #[test]
    fn mute_user() {
        let route = Route::MuteUser { user_id: USER_ID };

        assert_eq!(route.to_string(), "/users/5/mute");
    }

    #[test]
    fn oauth_email_request() {
        let route = Route::OAuthEmailRequest;

        assert_eq!(route.to_string(), "/oauth/emailrequest");
    }

    #[test]
    fn oauth_email_exchange() {
        let route = Route::OAuthEmailExchange;

        assert_eq!(route.to_string(), "/oauth/emailexchange");
    }

    #[test]
    fn oauth_logout() {
        let route = Route::OAuthLogout;

        assert_eq!(route.to_string(), "/oauth/logout");
    }

    #[test]
    fn rate_mod() {
        let route = Route::RateMod {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(route.to_string(), "/games/1/mods/2/ratings");
    }

    #[test]
    fn rename_game_tags() {
        let route = Route::RenameGameTags { game_id: GAME_ID };

        assert_eq!(route.to_string(), "/games/1/tags/rename");
    }

    #[test]
    fn reorder_mod_media() {
        let route = Route::ReorderModMedia {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(route.to_string(), "/games/1/mods/2/media/reorder");
    }

    #[test]
    fn submit_report() {
        let route = Route::SubmitReport;

        assert_eq!(route.to_string(), "/report");
    }

    #[test]
    fn subscribe_to_mod() {
        let route = Route::SubscribeToMod {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(route.to_string(), "/games/1/mods/2/subscribe");
    }

    #[test]
    fn terms() {
        let route = Route::Terms;

        assert_eq!(route.to_string(), "/authenticate/terms");
    }

    #[test]
    fn unmute_user() {
        let route = Route::UnmuteUser { user_id: USER_ID };

        assert_eq!(route.to_string(), "/users/5/mute");
    }

    #[test]
    fn unsubscribe_from_mod() {
        let route = Route::UnsubscribeFromMod {
            game_id: GAME_ID,
            mod_id: MOD_ID,
        };

        assert_eq!(route.to_string(), "/games/1/mods/2/subscribe");
    }

    #[test]
    fn user_authenticated() {
        let route = Route::UserAuthenticated;

        assert_eq!(route.to_string(), "/me");
    }

    #[test]
    fn user_events() {
        let route = Route::UserEvents;

        assert_eq!(route.to_string(), "/me/events");
    }

    #[test]
    fn user_files() {
        let route = Route::UserFiles;

        assert_eq!(route.to_string(), "/me/files");
    }

    #[test]
    fn user_games() {
        let route = Route::UserGames;

        assert_eq!(route.to_string(), "/me/games");
    }

    #[test]
    fn user_mods() {
        let route = Route::UserMods;

        assert_eq!(route.to_string(), "/me/mods");
    }

    #[test]
    fn user_muted() {
        let route = Route::UserMuted;

        assert_eq!(route.to_string(), "/me/users/muted");
    }

    #[test]
    fn user_ratings() {
        let route = Route::UserRatings;

        assert_eq!(route.to_string(), "/me/ratings");
    }

    #[test]
    fn user_subscriptions() {
        let route = Route::UserSubscriptions;

        assert_eq!(route.to_string(), "/me/subscribed");
    }
}
