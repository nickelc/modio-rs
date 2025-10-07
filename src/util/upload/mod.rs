//! Upload interface for mod files.

mod byte_ranges;
mod error;
mod multipart;

/// Required size (50MB) of upload parts except the last part.
pub const MULTIPART_FILE_PART_SIZE: u64 = 50 * 1024 * 1024;

pub use crate::request::files::multipart::ContentRange;
pub use byte_ranges::{byte_ranges, ByteRanges};
pub use error::{Error, ErrorKind};
pub use multipart::MultipartUploader;

use crate::client::Client;
use crate::types::files::multipart::UploadId;
use crate::types::id::{GameId, ModId};
use multipart::{Init, Started};

/// Extension trait for uploading files in multiple parts.
pub trait MultipartUpload: private::Sealed {
    /// Returns [`MultipartUploader`] for uploading files in multiple parts.
    ///
    /// # Example
    /// ```no_run
    /// # use modio::Client;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #     let client = Client::builder("MODIO_API_KEY".to_owned()).build()?;
    /// use modio::types::id::Id;
    /// use modio::util::upload::{self, ContentRange, MultipartUpload};
    /// use tokio::fs::File;
    /// use tokio::io::{AsyncReadExt, BufReader};
    /// use tokio_util::io::ReaderStream;
    ///
    /// let uploader = client
    ///     .upload(Id::new(51), Id::new(1234), "modfile.zip")
    ///     .nonce("xxxxxx") // Max 64 characters (Recommended: SHA-256)
    ///     .await?;
    ///
    /// let file = File::open("modfile.zip").await?;
    /// let file_size = file.metadata().await?.len();
    ///
    /// for (start, end) in upload::byte_ranges(file_size) {
    ///     let input = BufReader::new(file.try_clone().await?);
    ///     let part = input.take(upload::MULTIPART_FILE_PART_SIZE);
    ///     let stream = ReaderStream::new(part);
    ///
    ///     let range = ContentRange {
    ///         start,
    ///         end,
    ///         total: file_size,
    ///     };
    ///
    ///     // Add file part to the upload session.
    ///     uploader.add_part(range, stream).await?;
    /// }
    ///
    /// // Complete the multipart upload session.
    /// let uploader = uploader.complete().await?;
    ///
    /// // Finalize upload to the mod with file details.
    /// uploader.active(true).version("1.0").await?;
    /// #     Ok(())
    /// # }
    /// ```
    fn upload<'a>(
        &'a self,
        game_id: GameId,
        mod_id: ModId,
        filename: &'a str,
    ) -> MultipartUploader<'a, Init<'a>>;

    fn upload_for(
        &self,
        game_id: GameId,
        mod_id: ModId,
        upload_id: UploadId,
    ) -> MultipartUploader<'_, Started<'_>>;
}

impl MultipartUpload for Client {
    fn upload<'a>(
        &'a self,
        game_id: GameId,
        mod_id: ModId,
        filename: &'a str,
    ) -> MultipartUploader<'a, Init<'a>> {
        MultipartUploader::init(self, game_id, mod_id, filename)
    }

    fn upload_for(
        &self,
        game_id: GameId,
        mod_id: ModId,
        upload_id: UploadId,
    ) -> MultipartUploader<'_, Started<'_>> {
        MultipartUploader::started(self, game_id, mod_id, upload_id)
    }
}
mod private {
    use crate::Client;

    pub trait Sealed {}

    impl Sealed for Client {}
}
