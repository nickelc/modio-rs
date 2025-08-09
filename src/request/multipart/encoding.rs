use std::borrow::Cow;

use percent_encoding::utf8_percent_encode as percent_encode;
use percent_encoding::{AsciiSet, NON_ALPHANUMERIC};

// https://url.spec.whatwg.org/#fragment-percent-encode-set
const FRAGMENT_ENCODE_SET: &AsciiSet = &percent_encoding::CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'<')
    .add(b'>')
    .add(b'`');

// https://url.spec.whatwg.org/#path-percent-encode-set
const PATH_ENCODE_SET: &AsciiSet = &FRAGMENT_ENCODE_SET.add(b'#').add(b'?').add(b'{').add(b'}');

const PATH_SEGMENT_ENCODE_SET: &AsciiSet = &PATH_ENCODE_SET.add(b'/').add(b'%');

// https://tools.ietf.org/html/rfc8187#section-3.2.1
const ATTR_CHAR_ENCODE_SET: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'!')
    .remove(b'#')
    .remove(b'$')
    .remove(b'&')
    .remove(b'+')
    .remove(b'-')
    .remove(b'.')
    .remove(b'^')
    .remove(b'_')
    .remove(b'`')
    .remove(b'|')
    .remove(b'~');

#[inline]
pub fn percent_encode_path_segment(input: &str) -> Cow<'_, str> {
    percent_encode(input, PATH_SEGMENT_ENCODE_SET).into()
}

#[inline]
pub fn percent_encode_attr_char(input: &str) -> Cow<'_, str> {
    percent_encode(input, ATTR_CHAR_ENCODE_SET).into()
}

#[inline]
pub fn percent_encode_noop(input: &str) -> Cow<'_, str> {
    Cow::Borrowed(input)
}
