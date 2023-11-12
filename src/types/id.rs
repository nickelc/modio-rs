//! Type-safe ID type for each resource.

use std::cmp::Ordering;
use std::fmt::{self, Write};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::num::{NonZeroU64, ParseIntError, TryFromIntError};
use std::str::FromStr;

/// ID with a game marker.
pub type GameId = Id<marker::GameMarker>;
/// ID with a mod marker.
pub type ModId = Id<marker::ModMarker>;
/// ID with a file marker.
pub type FileId = Id<marker::FileMarker>;
/// ID with an event marker.
pub type EventId = Id<marker::EventMarker>;
/// ID with a comment marker.
pub type CommentId = Id<marker::CommentMarker>;
/// ID with a user marker.
pub type UserId = Id<marker::UserMarker>;
/// ID with a team member marker.
pub type MemberId = Id<marker::MemberMarker>;
/// ID with a resource marker.
pub type ResourceId = Id<marker::ResourceMarker>;

/// Markers for various resource types.
pub mod marker {
    /// Marker for game IDs.
    #[non_exhaustive]
    pub struct GameMarker;

    /// Marker for mod IDs.
    #[non_exhaustive]
    pub struct ModMarker;

    /// Marker for file IDs.
    #[non_exhaustive]
    pub struct FileMarker;

    /// Marker for event IDs.
    #[non_exhaustive]
    pub struct EventMarker;

    /// Marker for comment IDs.
    #[non_exhaustive]
    pub struct CommentMarker;

    /// Marker for user IDs.
    #[non_exhaustive]
    pub struct UserMarker;

    /// Marker for team member IDs.
    #[non_exhaustive]
    pub struct MemberMarker;

    /// Marker for resource IDs.
    #[non_exhaustive]
    pub struct ResourceMarker;
}

/// ID of a resource, such as the ID of a [game] or [mod].
///
/// [game]: crate::types::games::Game
/// [mod]: crate::types::mods::Mod
#[repr(transparent)]
pub struct Id<T> {
    phantom: PhantomData<fn(T) -> T>,
    value: NonZeroU64,
}

impl<T> Id<T> {
    const fn from_nonzero(value: NonZeroU64) -> Self {
        Self {
            phantom: PhantomData,
            value,
        }
    }

    /// Create a new ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use modio::types::id::marker::GameMarker;
    /// use modio::types::id::Id;
    ///
    /// let id: Id<GameMarker> = Id::new(123);
    ///
    /// // Using the provided type aliases.
    /// use modio::types::id::GameId;
    ///
    /// let game_id = GameId::new(123);
    ///
    /// assert_eq!(id, game_id);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the value is 0.
    #[track_caller]
    pub const fn new(value: u64) -> Self {
        if let Some(value) = Self::new_checked(value) {
            value
        } else {
            panic!("value is zero")
        }
    }

    /// Create a new ID if the given value is not zero.
    pub const fn new_checked(value: u64) -> Option<Self> {
        if let Some(value) = NonZeroU64::new(value) {
            Some(Self::from_nonzero(value))
        } else {
            None
        }
    }

    pub const fn get(self) -> u64 {
        self.value.get()
    }

    pub const fn transform<New>(self) -> Id<New> {
        Id::new(self.get())
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Id<T> {}

impl<T> fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Id")?;

        let name = std::any::type_name::<T>();
        if let Some(pos) = name.rfind("::") {
            if let Some(marker) = name.get(pos + 2..) {
                f.write_char('<')?;
                f.write_str(marker)?;
                f.write_char('>')?;
            }
        }

        f.write_char('(')?;
        fmt::Debug::fmt(&self.value, f)?;
        f.write_char(')')
    }
}

impl<T> fmt::Display for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.value, f)
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T> Eq for Id<T> {}

impl<T> PartialEq<u64> for Id<T> {
    fn eq(&self, other: &u64) -> bool {
        self.value.get() == *other
    }
}

impl<T> PartialEq<Id<T>> for u64 {
    fn eq(&self, other: &Id<T>) -> bool {
        other.value.get() == *self
    }
}

impl<T> PartialOrd for Id<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Id<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.value.get());
    }
}

impl<T> From<Id<T>> for u64 {
    fn from(value: Id<T>) -> Self {
        value.get()
    }
}

impl<T> From<NonZeroU64> for Id<T> {
    fn from(value: NonZeroU64) -> Self {
        Self::from_nonzero(value)
    }
}

impl<T> TryFrom<u64> for Id<T> {
    type Error = TryFromIntError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let value = NonZeroU64::try_from(value)?;
        Ok(Id::from_nonzero(value))
    }
}

impl<T> FromStr for Id<T> {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NonZeroU64::from_str(s).map(Self::from_nonzero)
    }
}

use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl<'de, T> Deserialize<'de> for Id<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        NonZeroU64::deserialize(deserializer).map(Self::from_nonzero)
    }
}

impl<T> Serialize for Id<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.value.serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::marker::*;
    use super::Id;

    #[test]
    fn from_str() {
        assert_eq!(
            Id::<GameMarker>::new(123),
            Id::<GameMarker>::from_str("123").unwrap()
        );
        assert!(Id::<GameMarker>::from_str("0").is_err());
        assert!(Id::<GameMarker>::from_str("123a").is_err());
    }
}
