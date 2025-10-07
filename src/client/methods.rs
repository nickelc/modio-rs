use std::path::Path;

use bytes::Bytes;
use futures_util::TryStream;

use crate::request::auth::external::Provider;
use crate::request::auth::{EmailExchange, EmailRequest, ExternalAuth, GetTerms, Logout};
use crate::request::files::multipart::{
    AddMultipartUploadFile, AddMultipartUploadPart, CompleteMultipartUploadSession, ContentRange,
    CreateMultipartUploadSession, DeleteMultipartUploadSession, GetMultipartUploadParts,
    GetMultipartUploadSessions,
};
use crate::request::files::{
    AddFile, DeleteFile, EditFile, GetFile, GetFiles, ManagePlatformStatus,
};
use crate::request::games::tags::{AddGameTags, DeleteGameTags, GetGameTags, RenameGameTag};
use crate::request::games::{AddGameMedia, GetGame, GetGameStats, GetGames};
use crate::request::mods::comments::{
    AddModComment, DeleteModComment, EditModComment, GetModComment, GetModComments,
    UpdateModCommentKarma,
};
use crate::request::mods::dependencies::{
    AddModDependencies, DeleteModDependencies, GetModDependencies,
};
use crate::request::mods::events::{GetModEvents, GetModsEvents};
use crate::request::mods::media::{AddModMedia, DeleteModMedia, ReorderModMedia};
use crate::request::mods::metadata::{AddModMetadata, DeleteModMetadata, GetModMetadata};
use crate::request::mods::stats::{GetModStats, GetModsStats};
use crate::request::mods::subscribe::{SubscribeToMod, UnsubscribeFromMod};
use crate::request::mods::tags::{AddModTags, DeleteModTags, GetModTags};
use crate::request::mods::{
    AddMod, DeleteMod, EditMod, GetMod, GetModTeamMembers, GetMods, SubmitModRating,
};
use crate::request::user::{
    GetAuthenticatedUser, GetMutedUsers, GetUserEvents, GetUserFiles, GetUserGames, GetUserMods,
    GetUserRatings, GetUserSubscriptions, MuteUser, UnmuteUser,
};
use crate::request::SubmitReport;
use crate::types::files::multipart::UploadId;
use crate::types::games::TagType;
use crate::types::id::{CommentId, FileId, GameId, ModId, ResourceId, UserId};
use crate::types::mods::MetadataMap;

use super::Client;

impl Client {
    /// Get text and links for user agreement and consent prior to authentication. [required: apikey]
    ///
    /// See the [mod.io docs](https://docs.mod.io/restapiref/#terms) for more information.
    pub const fn get_terms(&self) -> GetTerms<'_> {
        GetTerms::new(self)
    }

    /// Request a security code be sent to the email of the user. [required: apikey]
    pub const fn request_code<'a>(&'a self, email: &'a str) -> EmailRequest<'a> {
        EmailRequest::new(self, email)
    }

    /// Get the access token for a security code. [required: apikey]
    pub const fn request_token<'a>(&'a self, security_code: &'a str) -> EmailExchange<'a> {
        EmailExchange::new(self, security_code)
    }

    /// Authenticate via external services (Steam, GOG, Switch, Xbox, Discord, Oculus, Google etc.).
    ///
    /// See the [mod.io docs](https://docs.mod.io/restapiref/#authentication-2) for more information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use modio::Client;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #    let client = Client::builder("api-key".to_owned()).build()?;
    /// use modio::request::auth::external::Steam;
    /// let response = client.external_auth(Steam::new("ticket")).await?;
    /// let token = response.data().await?;
    ///
    /// use modio::request::auth::external::PSN;
    /// let response = client.external_auth(PSN::new("auth_code")).env(1).await?;
    /// let token = response.data().await?;
    ///
    /// use modio::request::auth::external::Discord;
    /// let response = client
    ///     .external_auth(Discord::new("token"))
    ///     .email("john@example.com")
    ///     .await?;
    /// let token = response.data().await?;
    /// #     Ok(())
    /// # }
    /// ```
    pub const fn external_auth<T: Provider>(&self, external: T) -> ExternalAuth<'_, T> {
        ExternalAuth::new(self, external)
    }

    /// Log out by revoking the current access token.
    pub const fn logout(&self) -> Logout<'_> {
        Logout::new(self)
    }
}

impl Client {
    /// Get all games.
    pub const fn get_games(&self) -> GetGames<'_> {
        GetGames::new(self)
    }

    /// Get a game.
    pub const fn get_game(&self, game_id: GameId) -> GetGame<'_> {
        GetGame::new(self, game_id)
    }

    /// Return the statistics for a game.
    pub const fn get_game_stats(&self, game_id: GameId) -> GetGameStats<'_> {
        GetGameStats::new(self, game_id)
    }

    /// Return the tag options for a game.
    pub const fn get_game_tags(&self, game_id: GameId) -> GetGameTags<'_> {
        GetGameTags::new(self, game_id)
    }

    /// Add tags which can applied to mods. [required: token]
    pub const fn add_game_tags<'a>(
        &'a self,
        game_id: GameId,
        name: &'a str,
        kind: TagType,
        tags: &'a [&'a str],
    ) -> AddGameTags<'a> {
        AddGameTags::new(self, game_id, name, kind, tags)
    }

    /// Delete an entire group of tags or individual tags. [required: token]
    pub const fn delete_game_tags<'a>(
        &'a self,
        game_id: GameId,
        name: &'a str,
    ) -> DeleteGameTags<'a> {
        DeleteGameTags::new(self, game_id, name)
    }

    /// Rename an existing tag, updating all mods in the progress. [required: token]
    pub const fn rename_game_tag<'a>(
        &'a self,
        game_id: GameId,
        from: &'a str,
        to: &'a str,
    ) -> RenameGameTag<'a> {
        RenameGameTag::new(self, game_id, from, to)
    }

    /// Add new media to a game. [required: token]
    pub const fn add_game_media(&self, game_id: GameId) -> AddGameMedia<'_> {
        AddGameMedia::new(self, game_id)
    }
}

impl Client {
    /// Get all mods for a game.
    ///
    /// See [Filters and sorting](crate::request::mods::filters)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #     let client = modio::Client::builder("key".to_owned()).build()?;
    /// use modio::request::filter::prelude::*;
    /// use modio::request::mods::filters::Name;
    /// use modio::types::id::Id;
    ///
    /// let list = client
    ///     .get_mods(Id::new(51))
    ///     .filter(Name::eq("the-x-com-files"))
    ///     .await?
    ///     .data()
    ///     .await?;
    /// #     Ok(())
    /// # }
    /// ```
    pub const fn get_mods(&self, game_id: GameId) -> GetMods<'_> {
        GetMods::new(self, game_id)
    }

    /// Get the Modio mod object that this refers to.
    pub const fn get_mod(&self, game_id: GameId, mod_id: ModId) -> GetMod<'_> {
        GetMod::new(self, game_id, mod_id)
    }

    /// Add a mod and return the newly created Modio mod object. [required: token]
    pub fn add_mod<'a>(
        &'a self,
        game_id: GameId,
        name: &'a str,
        summary: &'a str,
        logo: impl AsRef<Path>,
    ) -> AddMod<'a> {
        AddMod::new(self, game_id, name, summary, logo)
    }

    /// Edit details for a mod. [required: token]
    pub const fn edit_mod(&self, game_id: GameId, mod_id: ModId) -> EditMod<'_> {
        EditMod::new(self, game_id, mod_id)
    }

    /// Delete a mod. [required: token]
    pub const fn delete_mod(&self, game_id: GameId, mod_id: ModId) -> DeleteMod<'_> {
        DeleteMod::new(self, game_id, mod_id)
    }

    /// Add new media to a mod. [required: token]
    pub const fn add_mod_media(&self, game_id: GameId, mod_id: ModId) -> AddModMedia<'_> {
        AddModMedia::new(self, game_id, mod_id)
    }

    /// Delete media from a mod. [required: token]
    pub const fn delete_mod_media(&self, game_id: GameId, mod_id: ModId) -> DeleteModMedia<'_> {
        DeleteModMedia::new(self, game_id, mod_id)
    }

    /// Reorder the media of a mod. [required: token]
    pub const fn reorder_mod_media(&self, game_id: GameId, mod_id: ModId) -> ReorderModMedia<'_> {
        ReorderModMedia::new(self, game_id, mod_id)
    }

    /// Subscribe the authenticated user to a mod. [required: token]
    pub const fn subscribe_to_mod(&self, game_id: GameId, mod_id: ModId) -> SubscribeToMod<'_> {
        SubscribeToMod::new(self, game_id, mod_id)
    }

    /// Unsubscribe the authenticated user from a mod. [required: token]
    pub const fn unsubscribe_from_mod(
        &self,
        game_id: GameId,
        mod_id: ModId,
    ) -> UnsubscribeFromMod<'_> {
        UnsubscribeFromMod::new(self, game_id, mod_id)
    }

    /// Submit a rating for a mod. [required: token]
    pub const fn rate_mod(&self, game_id: GameId, mod_id: ModId) -> SubmitModRating<'_> {
        SubmitModRating::new(self, game_id, mod_id)
    }

    /// Get all users that are part of a mod team.
    pub const fn get_mod_team_members(
        &self,
        game_id: GameId,
        mod_id: ModId,
    ) -> GetModTeamMembers<'_> {
        GetModTeamMembers::new(self, game_id, mod_id)
    }
}

impl Client {
    /// Get all tags for a mod.
    pub const fn get_mod_tags(&self, game_id: GameId, mod_id: ModId) -> GetModTags<'_> {
        GetModTags::new(self, game_id, mod_id)
    }

    /// Add tags to a mod. [required: token]
    pub const fn add_mod_tags<'a>(
        &'a self,
        game_id: GameId,
        mod_id: ModId,
        tags: &'a [&'a str],
    ) -> AddModTags<'a> {
        AddModTags::new(self, game_id, mod_id, tags)
    }

    /// Delete tags from a mod. [required: token]
    pub const fn delete_mod_tags<'a>(
        &'a self,
        game_id: GameId,
        mod_id: ModId,
        tags: &'a [&'a str],
    ) -> DeleteModTags<'a> {
        DeleteModTags::new(self, game_id, mod_id, tags)
    }
}

impl Client {
    /// Get all mod events for a game sorted by latest first.
    pub const fn get_mods_events(&self, game_id: GameId) -> GetModsEvents<'_> {
        GetModsEvents::new(self, game_id)
    }

    /// Get all mod statistics for a game.
    pub const fn get_mods_stats(&self, game_id: GameId) -> GetModsStats<'_> {
        GetModsStats::new(self, game_id)
    }

    /// Get the event log for a mod, showing changes made sorted by latest first.
    pub const fn get_mod_events(&self, game_id: GameId, mod_id: ModId) -> GetModEvents<'_> {
        GetModEvents::new(self, game_id, mod_id)
    }

    /// Get the statistics for a mod.
    pub const fn get_mod_stats(&self, game_id: GameId, mod_id: ModId) -> GetModStats<'_> {
        GetModStats::new(self, game_id, mod_id)
    }
}

impl Client {
    /// Get all metadata key value pairs.
    pub const fn get_mod_metadata(&self, game_id: GameId, mod_id: ModId) -> GetModMetadata<'_> {
        GetModMetadata::new(self, game_id, mod_id)
    }

    /// Add metadata for a mod as key value pairs. [required: token]
    pub const fn add_mod_metadata(
        &self,
        game_id: GameId,
        mod_id: ModId,
        metadata: MetadataMap,
    ) -> AddModMetadata<'_> {
        AddModMetadata::new(self, game_id, mod_id, metadata)
    }

    /// Delete metadata key value pairs for a mod. [required: token]
    pub const fn delete_mod_metadata(
        &self,
        game_id: GameId,
        mod_id: ModId,
        metadata: MetadataMap,
    ) -> DeleteModMetadata<'_> {
        DeleteModMetadata::new(self, game_id, mod_id, metadata)
    }
}

impl Client {
    /// Get all dependencies a mod has selected.
    pub const fn get_mod_dependencies(
        &self,
        game_id: GameId,
        mod_id: ModId,
    ) -> GetModDependencies<'_> {
        GetModDependencies::new(self, game_id, mod_id)
    }

    /// Add mod dependencies required by a mod. [required: token]
    pub const fn add_mod_dependencies<'a>(
        &'a self,
        game_id: GameId,
        mod_id: ModId,
        deps: &'a [ModId],
    ) -> AddModDependencies<'a> {
        AddModDependencies::new(self, game_id, mod_id, deps)
    }

    /// Delete mod dependencies a mod requires. [required: token]
    pub const fn delete_mod_dependencies<'a>(
        &'a self,
        game_id: GameId,
        mod_id: ModId,
        deps: &'a [ModId],
    ) -> DeleteModDependencies<'a> {
        DeleteModDependencies::new(self, game_id, mod_id, deps)
    }
}

impl Client {
    /// Get all comments posted on a mod's profile.
    pub const fn get_mod_comments(&self, game_id: GameId, mod_id: ModId) -> GetModComments<'_> {
        GetModComments::new(self, game_id, mod_id)
    }

    /// Get a comment posted on a mod's profile.
    pub const fn get_mod_comment(
        &self,
        game_id: GameId,
        mod_id: ModId,
        comment_id: CommentId,
    ) -> GetModComment<'_> {
        GetModComment::new(self, game_id, mod_id, comment_id)
    }

    /// Add a comment on a mod's profile. [required: token]
    pub const fn add_mod_comment<'a>(
        &'a self,
        game_id: GameId,
        mod_id: ModId,
        content: &'a str,
    ) -> AddModComment<'a> {
        AddModComment::new(self, game_id, mod_id, content)
    }

    /// Edit a comment on a mod's profile. [required: token]
    pub const fn edit_mod_comment<'a>(
        &'a self,
        game_id: GameId,
        mod_id: ModId,
        comment_id: CommentId,
        content: &'a str,
    ) -> EditModComment<'a> {
        EditModComment::new(self, game_id, mod_id, comment_id, content)
    }

    /// Delete a comment from a mod's profile. [required: token]
    pub const fn delete_mod_comment(
        &self,
        game_id: GameId,
        mod_id: ModId,
        comment_id: CommentId,
    ) -> DeleteModComment<'_> {
        DeleteModComment::new(self, game_id, mod_id, comment_id)
    }

    /// Update the karma rating for a comment. [required: token]
    pub const fn update_mod_comment_karma(
        &self,
        game_id: GameId,
        mod_id: ModId,
        comment_id: CommentId,
    ) -> UpdateModCommentKarma<'_> {
        UpdateModCommentKarma::new(self, game_id, mod_id, comment_id)
    }
}

impl Client {
    /// Get all files from a mod.
    pub const fn get_files(&self, game_id: GameId, mod_id: ModId) -> GetFiles<'_> {
        GetFiles::new(self, game_id, mod_id)
    }

    /// Get a file from a mod.
    pub const fn get_file(&self, game_id: GameId, mod_id: ModId, file_id: FileId) -> GetFile<'_> {
        GetFile::new(self, game_id, mod_id, file_id)
    }

    /// Add file to a mod. [required: token]
    pub const fn add_file(&self, game_id: GameId, mod_id: ModId) -> AddFile<'_> {
        AddFile::new(self, game_id, mod_id)
    }

    /// Edit the details of a published file. [required: token]
    pub const fn edit_file(&self, game_id: GameId, mod_id: ModId, file_id: FileId) -> EditFile<'_> {
        EditFile::new(self, game_id, mod_id, file_id)
    }

    /// Delete a mod file. [required: token]
    pub const fn delete_file(
        &self,
        game_id: GameId,
        mod_id: ModId,
        file_id: FileId,
    ) -> DeleteFile<'_> {
        DeleteFile::new(self, game_id, mod_id, file_id)
    }

    /// Manage the platform status of a particular mod file. [required: token]
    pub const fn manage_platform_status(
        &self,
        game_id: GameId,
        mod_id: ModId,
        file_id: FileId,
    ) -> ManagePlatformStatus<'_> {
        ManagePlatformStatus::new(self, game_id, mod_id, file_id)
    }
}

impl Client {
    /// Get all upload sessions. [required: token]
    pub const fn get_multipart_upload_sessions(
        &self,
        game_id: GameId,
        mod_id: ModId,
    ) -> GetMultipartUploadSessions<'_> {
        GetMultipartUploadSessions::new(self, game_id, mod_id)
    }

    /// Create a new multipart upload session. [required: token]
    pub const fn create_multipart_upload_session<'a>(
        &'a self,
        game_id: GameId,
        mod_id: ModId,
        filename: &'a str,
    ) -> CreateMultipartUploadSession<'a> {
        CreateMultipartUploadSession::new(self, game_id, mod_id, filename)
    }

    /// Get the upload parts of the session. [required: token]
    pub const fn get_multipart_upload_parts(
        &self,
        game_id: GameId,
        mod_id: ModId,
        upload_id: UploadId,
    ) -> GetMultipartUploadParts<'_> {
        GetMultipartUploadParts::new(self, game_id, mod_id, upload_id)
    }

    /// Add a new part to an existing upload session. [required: token]
    pub const fn add_multipart_upload_part<S>(
        &self,
        game_id: GameId,
        mod_id: ModId,
        upload_id: UploadId,
        range: ContentRange,
        stream: S,
    ) -> AddMultipartUploadPart<'_, S>
    where
        S: TryStream + Send + 'static,
        S::Ok: Into<Bytes>,
        S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        AddMultipartUploadPart::new(self, game_id, mod_id, upload_id, range, stream)
    }

    /// Complete an active upload session after uploading all parts with
    /// [`Client::add_multipart_upload_part`]. [required: token]
    pub const fn complete_multipart_upload_session(
        &self,
        game_id: GameId,
        mod_id: ModId,
        upload_id: UploadId,
    ) -> CompleteMultipartUploadSession<'_> {
        CompleteMultipartUploadSession::new(self, game_id, mod_id, upload_id)
    }

    /// Terminate an active upload session. [required: token]
    pub const fn delete_multipart_upload_session(
        &self,
        game_id: GameId,
        mod_id: ModId,
        upload_id: UploadId,
    ) -> DeleteMultipartUploadSession<'_> {
        DeleteMultipartUploadSession::new(self, game_id, mod_id, upload_id)
    }

    /// Finalize the upload with file details. [required: token]
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use modio::types::files::multipart::{UploadId, UploadSession};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #     let client = modio::Client::builder("key".to_owned()).build()?;
    /// #     let session: UploadSession = unimplemented!();
    /// use modio::types::files::File;
    /// use modio::types::id::Id;
    ///
    /// let game_id = Id::new(51);
    /// let mod_id = Id::new(1041);
    /// let UploadSession { id: upload_id, .. } = session;
    ///
    /// let file: File = client
    ///     .add_multipart_upload_file(game_id, mod_id, upload_id)
    ///     .version("1.0")
    ///     .active(true)
    ///     .await?
    ///     .data()
    ///     .await?;
    /// #     Ok(())
    /// # }
    /// ```
    pub const fn add_multipart_upload_file(
        &self,
        game_id: GameId,
        mod_id: ModId,
        upload_id: UploadId,
    ) -> AddMultipartUploadFile<'_> {
        AddMultipartUploadFile::new(self, game_id, mod_id, upload_id)
    }
}

impl Client {
    /// Get the authenticated user details. [required: token]
    pub const fn get_authenticated_user(&self) -> GetAuthenticatedUser<'_> {
        GetAuthenticatedUser::new(self)
    }

    /// Get all games the authenticated user added or is a team member of. [required: token]
    pub const fn get_user_games(&self) -> GetUserGames<'_> {
        GetUserGames::new(self)
    }

    /// Get all mods the authenticated user added or is a team member of. [required: token]
    pub const fn get_user_mods(&self) -> GetUserMods<'_> {
        GetUserMods::new(self)
    }

    /// Get all mod files the authenticated user uploaded. [required: token]
    pub const fn get_user_files(&self) -> GetUserFiles<'_> {
        GetUserFiles::new(self)
    }

    /// Get all mod ratings the authenticated user submitted. [required: token]
    pub const fn get_user_ratings(&self) -> GetUserRatings<'_> {
        GetUserRatings::new(self)
    }

    /// Get all mods the authenticated user is subscribed to. [required: token]
    pub const fn get_user_subscriptions(&self) -> GetUserSubscriptions<'_> {
        GetUserSubscriptions::new(self)
    }

    /// Get events that have been fired specific to the user. [required: token]
    pub const fn get_user_events(&self) -> GetUserEvents<'_> {
        GetUserEvents::new(self)
    }
}

impl Client {
    /// Get all user muted by the authenticated user. [required: token]
    pub const fn get_muted_users(&self) -> GetMutedUsers<'_> {
        GetMutedUsers::new(self)
    }

    /// Mute a user. [required: token]
    pub const fn mute_user(&self, user_id: UserId) -> MuteUser<'_> {
        MuteUser::new(self, user_id)
    }

    /// Unmute a previously muted user. [required: token]
    pub const fn unmute_user(&self, user_id: UserId) -> UnmuteUser<'_> {
        UnmuteUser::new(self, user_id)
    }
}

impl Client {
    /// Report a resource (game, guide, mod or user) on mod.io.
    pub fn report<'a>(
        &'a self,
        resource: &'a str,
        id: ResourceId,
        kind: u8,
        summary: &'a str,
    ) -> SubmitReport<'a> {
        SubmitReport::new(self, resource, id, kind, summary)
    }
}
