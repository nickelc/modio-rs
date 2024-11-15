use serde::de::{DeserializeOwned, Error, MapAccess};

mod smallstr {
    use std::fmt;

    use serde::de::{Deserialize, Deserializer, Error, Visitor};
    use serde::ser::{Serialize, Serializer};

    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SmallStr<const LENGTH: usize> {
        bytes: [u8; LENGTH],
    }

    impl<const LENGTH: usize> SmallStr<LENGTH> {
        pub(crate) const fn from_str(input: &str) -> Option<Self> {
            if input.len() > LENGTH {
                return None;
            }
            Some(Self::from_bytes(input.as_bytes()))
        }

        pub(crate) const fn from_bytes(input: &[u8]) -> Self {
            let mut bytes = [0; LENGTH];
            let mut idx = 0;

            while idx < input.len() {
                bytes[idx] = input[idx];
                idx += 1;
            }

            Self { bytes }
        }

        pub fn as_str(&self) -> &str {
            std::str::from_utf8(&self.bytes)
                .expect("invalid utf8 string")
                .trim_end_matches('\0')
        }
    }

    impl<const LENGTH: usize> fmt::Debug for SmallStr<LENGTH> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(self.as_str())
        }
    }

    impl<'de, const LENGTH: usize> Deserialize<'de> for SmallStr<LENGTH> {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            struct StrVisitor<const LENGTH: usize>;

            impl<'de, const LENGTH: usize> Visitor<'de> for StrVisitor<LENGTH> {
                type Value = SmallStr<LENGTH>;

                fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                    formatter.write_str("string")
                }

                fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
                    SmallStr::from_str(v).ok_or_else(|| Error::custom("string is too long"))
                }

                fn visit_string<E: Error>(self, v: String) -> Result<Self::Value, E> {
                    SmallStr::from_str(&v).ok_or_else(|| Error::custom("string is too long"))
                }
            }

            deserializer.deserialize_any(StrVisitor::<LENGTH>)
        }
    }

    impl<const LENGTH: usize> Serialize for SmallStr<LENGTH> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.as_str().serialize(serializer)
        }
    }
}
pub use smallstr::SmallStr;

pub mod url {
    use std::fmt;

    use serde::de::{Deserializer, Error, Visitor};
    use url::Url;

    struct UrlVisitor;

    impl Visitor<'_> for UrlVisitor {
        type Value = Url;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("a string representing an URL")
        }

        fn visit_str<E: Error>(self, s: &str) -> Result<Self::Value, E> {
            Url::parse(s).map_err(|err| Error::custom(format!("{err}: {s:?}")))
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Url, D::Error> {
        deserializer.deserialize_any(UrlVisitor)
    }

    pub mod opt {
        use std::fmt;

        use serde::de::{Deserializer, Error, Visitor};
        use url::Url;

        struct UrlVisitor;

        impl<'de> Visitor<'de> for UrlVisitor {
            type Value = Option<Url>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an optional string representing an URL")
            }

            fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
                d.deserialize_any(super::UrlVisitor).map(Some)
            }

            fn visit_none<E: Error>(self) -> Result<Self::Value, E> {
                Ok(None)
            }

            fn visit_unit<E: Error>(self) -> Result<Self::Value, E> {
                Ok(None)
            }
        }

        pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Url>, D::Error> {
            d.deserialize_option(UrlVisitor)
        }
    }
}

pub trait DeserializeField<T: DeserializeOwned> {
    fn deserialize_value<'de, A: MapAccess<'de>>(
        &mut self,
        name: &'static str,
        map: &mut A,
    ) -> Result<(), A::Error>;
}

impl<T: DeserializeOwned> DeserializeField<T> for Option<T> {
    fn deserialize_value<'de, A>(&mut self, name: &'static str, map: &mut A) -> Result<(), A::Error>
    where
        A: MapAccess<'de>,
    {
        if self.is_some() {
            return Err(A::Error::duplicate_field(name));
        }
        self.replace(map.next_value()?);
        Ok(())
    }
}

pub trait MissingField<T> {
    fn missing_field<E: Error>(self, name: &'static str) -> Result<T, E>;
}

impl<T> MissingField<T> for Option<T> {
    fn missing_field<E: Error>(self, name: &'static str) -> Result<T, E> {
        self.ok_or_else(|| Error::missing_field(name))
    }
}
