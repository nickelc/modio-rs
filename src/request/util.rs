use serde::ser::{Serialize, SerializeMap, Serializer};

pub struct ArrayParams<'a, T> {
    name: &'static str,
    items: &'a [T],
}

impl<'a, T> ArrayParams<'a, T> {
    pub const fn new(name: &'static str, items: &'a [T]) -> Self {
        Self { name, items }
    }
}

impl<T: Serialize> Serialize for ArrayParams<'_, T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.items.len()))?;
        for item in self.items {
            map.serialize_entry(self.name, item)?;
        }
        map.end()
    }
}

#[cfg(test)]
mod tests {
    use serde_test::{assert_ser_tokens, Token};

    use super::ArrayParams;

    #[test]
    pub fn serialize_array_params() {
        let params = ArrayParams::new("foo[]", &["aaa", "bbb", "ccc"]);

        assert_ser_tokens(
            &params,
            &[
                Token::Map { len: Some(3) },
                Token::Str("foo[]"),
                Token::Str("aaa"),
                Token::Str("foo[]"),
                Token::Str("bbb"),
                Token::Str("foo[]"),
                Token::Str("ccc"),
                Token::MapEnd,
            ],
        );
    }
}
