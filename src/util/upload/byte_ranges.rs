use std::iter::{repeat, Chain, Map, Repeat, Zip};
use std::ops::Range;

use super::MULTIPART_FILE_PART_SIZE;

/// The [`ByteRanges`] type implements the underlying `Iterator` for [`byte_ranges`].
///
/// # Example
///
/// ```
/// # use modio::util::upload::ByteRanges;
/// let mut iter = ByteRanges::new(14).chunks(5).into_iter();
///
/// assert_eq!(iter.next(), Some((0, 4)));
/// assert_eq!(iter.next(), Some((5, 9)));
/// assert_eq!(iter.next(), Some((10, 13)));
/// assert_eq!(iter.next(), None);
/// ```
pub struct ByteRanges {
    length: u64,
    chunk_size: u64,
}

impl ByteRanges {
    /// Creates a new `ByteRanges` for the give length.
    pub const fn new(length: u64) -> Self {
        Self {
            length,
            chunk_size: MULTIPART_FILE_PART_SIZE,
        }
    }

    /// Sets the chunk size for the iterator. Defaults to [`MULTIPART_FILE_PART_SIZE`].
    pub const fn chunks(mut self, chunk_size: u64) -> Self {
        self.chunk_size = chunk_size;
        self
    }
}

impl IntoIterator for ByteRanges {
    type Item = (u64, u64);
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        fn map_fn((i, chunk_size): (u64, u64)) -> (u64, u64) {
            (i * chunk_size, (i + 1) * chunk_size - 1)
        }

        let ByteRanges { length, chunk_size } = self;

        let count = length / chunk_size;
        let rem = length % chunk_size;

        let iter = (0..count)
            .zip(repeat(chunk_size))
            .map(map_fn as MapFn)
            .chain(Some((count * chunk_size, count * chunk_size + rem - 1)));

        IntoIter { inner: iter }
    }
}

type MapFn = fn((u64, u64)) -> (u64, u64);
type Elements = Map<Zip<Range<u64>, Repeat<u64>>, MapFn>;
type Last = std::option::IntoIter<(u64, u64)>;

pub struct IntoIter {
    inner: Chain<Elements, Last>,
}

impl Iterator for IntoIter {
    type Item = (u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// Calculates bytes ranges for the given `length` parameter and the chunk size of 50MB.
///
/// # Example
///
/// ```
/// # use modio::util::upload::byte_ranges;
/// let mut iter = byte_ranges(52 * 1024 * 1024);
///
/// assert_eq!(iter.next(), Some((0, 52428799)));
/// assert_eq!(iter.next(), Some((52428800, 54525951)));
/// assert_eq!(iter.next(), None);
/// ```
pub fn byte_ranges(length: u64) -> impl Iterator<Item = (u64, u64)> {
    ByteRanges::new(length).into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn multiple_byte_ranges() {
        const SIZE: u64 = 522 * 1024 * 1024;

        let ranges: Vec<_> = byte_ranges(SIZE).collect();

        assert_eq!(
            ranges,
            [
                (0, 52428799),
                (52428800, 104857599),
                (104857600, 157286399),
                (157286400, 209715199),
                (209715200, 262143999),
                (262144000, 314572799),
                (314572800, 367001599),
                (367001600, 419430399),
                (419430400, 471859199),
                (471859200, 524287999),
                (524288000, 547356671),
            ]
        );
    }

    #[test]
    pub fn single_byte_range() {
        const SIZE: u64 = 25 * 1024 * 1024;

        let ranges: Vec<_> = byte_ranges(SIZE).collect();

        assert_eq!(ranges, [(0, SIZE - 1)]);
    }
}
