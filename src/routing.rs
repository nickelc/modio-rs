use std::borrow::Cow;

use reqwest::Method;

use crate::types::id::{CommentId, FileId, GameId, ModId, UserId};

// macro: define_routes {{{
// Based on https://stackoverflow.com/a/44161783
macro_rules! define_routes {
    // Start rule.
    // Note: `$(,)*` is a trick to eat any number of trailing commas.
    ( $( {$($route:tt)*} ),* $(,)*) => {
        // This starts the parse, giving the initial state of the output
        // (i.e. empty).  Note that the route come after the semicolon.
        define_routes! { @parse {}, {}; $({$($route)*},)* }
    };

    // Termination rule: no more input.
    (
        @parse
        // $eout will be the body of the enum.
        {$($eout:tt)*},
        // $pout will be the body of the `pieces` match.
        {$($pout:tt)*};
        // See, nothing here?
    ) => {
        #[derive(Debug, Clone, Copy)]
        pub enum Route {
            $($eout)*
        }

        impl Route {
            pub fn pieces(&self) -> (Method, Cow<'_, str>, AuthMethod) {
                match self {
                    $($pout)*
                }
            }
        }
    };

    // Rule for routes with no arguments.
    (
        @parse {$($eout:tt)*}, {$($pout:tt)*};
        {
            $rname:ident,
            $method:ident: $path:expr,
            $required:ident $(,)*
        },
        $($tail:tt)*
    ) => {
        define_routes! {
            @parse
            {
                $($eout)*
                $rname,
            },
            {
                $($pout)*
                Route::$rname => {
                    (Method::$method, Cow::from($path), AuthMethod::$required)
                },
            };
            $($tail)*
        }
    };

    // Rule for other routes.
    (
        @parse {$($eout:tt)*}, {$($pout:tt)*};
        {
            $rname:ident,
            $method:ident: $path:expr,
            [$($args:ident: $Type:ty),* $(,)*],
            $required:ident $(,)*
        },
        $($tail:tt)*
    ) => {
        define_routes! {
            @parse
            {
                $($eout)*
                $rname { $( $args: $Type, )* },
            },
            {
                $($pout)*
                Route::$rname {
                    $($args,)*
                }=> {
                    (Method::$method, Cow::from(format!($path, $($args,)*)), AuthMethod::$required)
                },
            };
            $($tail)*
        }
    };
}
// }}}

pub enum AuthMethod {
    ApiKey,
    Token,
    Any,
}

define_routes! {
    { AuthEmailRequest, POST: "/oauth/emailrequest", ApiKey },
    { AuthEmailExchange, POST: "/oauth/emailexchange", ApiKey },
    { AuthSteam, POST: "/external/steamauth", ApiKey },
    { AuthOculus, POST: "/external/oculusauth", ApiKey },
    { AuthSwitch, POST: "/external/switchauth", ApiKey },
    { AuthXbox, POST: "/external/xboxauth", ApiKey },
    { AuthDiscord, POST: "/external/discordauth", ApiKey },
    { AuthGoogle, POST: "/external/googleauth", ApiKey },
    { Terms, GET: "/authenticate/terms", ApiKey},
    { GetGames, GET: "/games", Any },
    { GetGame, GET: "/games/{}", [game_id: GameId], Any },
    { AddGameMedia, POST: "/games/{}/media", [game_id: GameId], Token },
    { GetGameStats, GET: "/games/{}/stats", [game_id: GameId], Any },
    { GetGameTags, GET: "/games/{}/tags", [game_id: GameId], Any },
    { AddGameTags, POST: "/games/{}/tags", [game_id: GameId], Token },
    { DeleteGameTags, DELETE: "/games/{}/tags", [game_id: GameId], Token },
    { GetMods, GET: "/games/{}/mods", [game_id: GameId], Any },
    { GetMod, GET: "/games/{}/mods/{}", [game_id: GameId, mod_id: ModId], Any },
    { AddMod, POST: "/games/{}/mods", [game_id: GameId], Token },
    { EditMod, PUT: "/games/{}/mods/{}", [game_id: GameId, mod_id: ModId], Token },
    { DeleteMod, DELETE: "/games/{}/mods/{}", [game_id: GameId, mod_id: ModId], Token },
    { AddModMedia, POST: "/games/{}/mods/{}/media", [game_id: GameId, mod_id: ModId], Token },
    { DeleteModMedia, DELETE: "/games/{}/mods/{}/media", [game_id: GameId, mod_id: ModId], Token },
    { Subscribe, POST: "/games/{}/mods/{}/subscribe", [game_id: GameId, mod_id: ModId], Token },
    { Unsubscribe, DELETE: "/games/{}/mods/{}/subscribe", [game_id: GameId, mod_id: ModId], Token },
    { GetAllModEvents, GET: "/games/{}/mods/events", [game_id: GameId], Any },
    { GetModEvents, GET: "/games/{}/mods/{}/events", [game_id: GameId, mod_id: ModId], Any },
    { GetAllModStats, GET: "/games/{}/mods/stats", [game_id: GameId], Any },
    { GetModStats, GET: "/games/{}/mods/{}/stats", [game_id: GameId, mod_id: ModId], Any },
    { GetModTags, GET: "/games/{}/mods/{}/tags", [game_id: GameId, mod_id: ModId], Any },
    { AddModTags, POST: "/games/{}/mods/{}/tags", [game_id: GameId, mod_id: ModId], Token },
    { DeleteModTags, DELETE: "/games/{}/mods/{}/tags", [game_id: GameId, mod_id: ModId], Token },
    { RateMod, POST: "/games/{}/mods/{}/ratings", [game_id: GameId, mod_id: ModId], Token },
    { GetModMetadata, GET: "/games/{}/mods/{}/metadatakvp", [game_id: GameId, mod_id: ModId], Any },
    { AddModMetadata, POST: "/games/{}/mods/{}/metadatakvp", [game_id: GameId, mod_id: ModId], Token },
    { DeleteModMetadata, DELETE: "/games/{}/mods/{}/metadatakvp", [game_id: GameId, mod_id: ModId], Token },
    { GetModDependencies, GET: "/games/{}/mods/{}/dependencies", [game_id: GameId, mod_id: ModId], Any },
    { AddModDependencies, POST: "/games/{}/mods/{}/dependencies", [game_id: GameId, mod_id: ModId], Token },
    { DeleteModDependencies, DELETE: "/games/{}/mods/{}/dependencies", [game_id: GameId, mod_id: ModId], Token },
    { GetTeamMembers, GET: "/games/{}/mods/{}/team", [game_id: GameId, mod_id: ModId], Any },
    { GetModComments, GET: "/games/{}/mods/{}/comments", [game_id: GameId, mod_id: ModId], Any },
    { GetModComment, GET: "/games/{}/mods/{}/comments/{}", [game_id: GameId, mod_id: ModId, comment_id: CommentId], Any },
    { AddModComment, POST: "/games/{}/mods/{}/comments", [game_id: GameId, mod_id: ModId], Token },
    { EditModComment, PUT: "/games/{}/mods/{}/comments/{}", [game_id: GameId, mod_id: ModId, comment_id: CommentId], Token },
    { DeleteModComment, DELETE: "/games/{}/mods/{}/comments/{}", [game_id: GameId, mod_id: ModId, comment_id: CommentId], Token },
    { AddModCommentKarma, POST: "/games/{}/mods/{}/comments/{}/karma", [game_id: GameId, mod_id: ModId, comment_id: CommentId], Token },
    { GetFiles, GET: "/games/{}/mods/{}/files", [game_id: GameId, mod_id: ModId], Any },
    { GetFile, GET: "/games/{}/mods/{}/files/{}", [game_id: GameId, mod_id: ModId, file_id: FileId], Any },
    { AddFile, POST: "/games/{}/mods/{}/files", [game_id: GameId, mod_id: ModId], Token },
    { EditFile, PUT: "/games/{}/mods/{}/files/{}", [game_id: GameId, mod_id: ModId, file_id: FileId], Token },
    { DeleteFile, DELETE: "/games/{}/mods/{}/files/{}", [game_id: GameId, mod_id: ModId, file_id: FileId], Token },
    { ManagePlatformStatus, POST: "/games/{}/mods/{}/files/{}/platforms", [game_id: GameId, mod_id: ModId, file_id: FileId], Token },
    { AuthorizedUser, GET: "/me", Token },
    { UserSubscriptions, GET: "/me/subscribed", Token },
    { UserEvents, GET: "/me/events", Token },
    { UserGames, GET: "/me/games", Token },
    { UserMods, GET: "/me/mods", Token },
    { UserFiles, GET: "/me/files", Token },
    { UserRatings, GET: "/me/ratings", Token },
    { UserMuted, GET: "/me/users/muted", Token },
    { MuteUser, POST: "/users/{}/mute", [user_id: UserId], Token },
    { UnmuteUser, DELETE: "/users/{}/mute", [user_id: UserId], Token },
    { SubmitReport, POST: "/report", Token },
}

#[cfg(test)]
mod tests {
    use super::Route;
    use crate::types::id::Id;

    #[test]
    fn pieces() {
        let route = Route::GetGames;
        let (_, path, _) = route.pieces();
        assert_eq!(path, "/games");

        let route = Route::GetFile {
            game_id: Id::new(1),
            mod_id: Id::new(2),
            file_id: Id::new(3),
        };
        let (_, path, _) = route.pieces();
        assert_eq!(path, "/games/1/mods/2/files/3");
    }
}

// vim: fdm=marker
