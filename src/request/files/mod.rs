pub mod multipart;

mod add_file;
mod delete_file;
mod edit_file;
mod get_file;
mod get_files;
mod manage_platform_status;

pub use add_file::AddFile;
pub use delete_file::DeleteFile;
pub use edit_file::EditFile;
pub use get_file::GetFile;
pub use get_files::GetFiles;
pub use manage_platform_status::ManagePlatformStatus;

/// Modfile filters and sorting.
///
/// # Filters
/// - `Fulltext`
/// - `Id`
/// - `ModId`
/// - `DateAdded`
/// - `DateScanned`
/// - `VirusStatus`
/// - `VirusPositive`
/// - `Filesize`
/// - `Filehash`
/// - `Filename`
/// - `Version`
/// - `Changelog`
///
/// # Sorting
/// - `Id`
/// - `ModId`
/// - `DateAdded`
/// - `Version`
///
/// See [modio docs](https://docs.mod.io/restapiref/#get-modfiles) for more information.
///
/// By default this returns up to `100` items. You can limit the result by using `limit` and
/// `offset`.
///
/// # Example
/// ```
/// use modio::request::filter::prelude::*;
/// use modio::request::files::filters::Id;
///
/// let filter = Id::_in(vec![1, 2]).order_by(Id::desc());
/// ```
#[rustfmt::skip]
pub mod filters {
    #[doc(inline)]
    pub use crate::request::filter::prelude::Fulltext;
    #[doc(inline)]
    pub use crate::request::filter::prelude::Id;
    #[doc(inline)]
    pub use crate::request::filter::prelude::ModId;
    #[doc(inline)]
    pub use crate::request::filter::prelude::DateAdded;

    filter!(DateScanned, DATE_SCANNED, "date_scanned", Eq, NotEq, In, Cmp);
    filter!(VirusStatus, VIRUS_STATUS, "virus_status", Eq, NotEq, In, Cmp);
    filter!(VirusPositive, VIRUS_POSITIVE, "virus_positive", Eq, NotEq, In, Cmp);
    filter!(Filesize, FILESIZE, "filesize", Eq, NotEq, In, Cmp, OrderBy);
    filter!(Filehash, FILEHASH, "filehash", Eq, NotEq, In, Like);
    filter!(Filename, FILENAME, "filename", Eq, NotEq, In, Like);
    filter!(Version, VERSION, "version", Eq, NotEq, In, Like, OrderBy);
    filter!(Changelog, CHANGELOG, "changelog", Eq, NotEq, In, Like);
}
