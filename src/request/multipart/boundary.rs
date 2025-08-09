use std::fmt::Write;

use bytes::{Bytes, BytesMut};

const BOUNDARY_LEN: usize = 16 * 4 + 3; // "{a:016x}-{b:016x}-{c:016x}-{d:016x}"
const BOUNDARY_DASHES: &[u8] = b"--";
const BOUNDARY_TERMINATOR: &[u8] = b"--\r\n";
const CRLF: &[u8] = b"\r\n";

pub struct Boundary(Bytes);

impl Boundary {
    pub const LEN: usize = BOUNDARY_LEN; // "{a:016x}-{b:016x}-{c:016x}-{d:016x}"

    pub fn generate() -> Self {
        let a = random();
        let b = random();
        let c = random();
        let d = random();

        let mut buf = BytesMut::with_capacity(Self::LEN);
        let _ = write!(&mut buf, "{a:016x}-{b:016x}-{c:016x}-{d:016x}");
        Self(buf.freeze())
    }

    pub fn value(&self) -> &[u8] {
        &self.0
    }

    pub fn delimiter(&self) -> Bytes {
        let capacity = BOUNDARY_DASHES.len() + Self::LEN + CRLF.len();
        let mut buf = BytesMut::with_capacity(capacity);
        buf.extend_from_slice(BOUNDARY_DASHES);
        buf.extend_from_slice(&self.0);
        buf.extend_from_slice(CRLF);
        buf.freeze()
    }

    pub fn terminator(&self) -> Bytes {
        let capacity = BOUNDARY_DASHES.len() + Self::LEN + BOUNDARY_TERMINATOR.len();
        let mut buf = BytesMut::with_capacity(capacity);
        buf.extend_from_slice(BOUNDARY_DASHES);
        buf.extend_from_slice(&self.0);
        buf.extend_from_slice(BOUNDARY_TERMINATOR);
        buf.freeze()
    }
}

fn random() -> u64 {
    use std::cell::Cell;
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    use std::num::Wrapping;

    thread_local! {
        static RNG: Cell<Wrapping<u64>> = Cell::new(Wrapping(seed()));
    }

    fn seed() -> u64 {
        let seed = RandomState::new();

        let mut out = 0;
        let mut cnt = 0;
        while out == 0 {
            cnt += 1;
            let mut hasher = seed.build_hasher();
            hasher.write_usize(cnt);
            out = hasher.finish();
        }
        out
    }

    RNG.with(|rng| {
        let mut n = rng.get();
        debug_assert_ne!(n.0, 0);
        n ^= n >> 12;
        n ^= n << 25;
        n ^= n >> 27;
        rng.set(n);
        n.0.wrapping_mul(0x2545_f491_4f6c_dd1d)
    })
}
