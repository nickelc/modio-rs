// macro: bitflags {{{
macro_rules! bitflags {
    (
        $(#[$outer:meta])*
        $vis:vis struct $BitFlags:ident: $T:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                const $Flag:ident = $value:expr;
            )*
        }

        $($t:tt)*
    ) => {
        bitflags::bitflags! {
            $(#[$outer])*
            $vis struct $BitFlags: $T {
                $(
                    $(#[$inner $($args)*])*
                    const $Flag = $value;
                )*
            }
        }

        bitflags!(__impl_display $BitFlags);
        bitflags!(__impl_serde $BitFlags: $T);

        bitflags! {
            $($t)*
        }
    };
    (__impl_display $BitFlags:ident) => {
        impl ::std::fmt::Display for $BitFlags {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                self.bits.fmt(f)
            }
        }
    };
    (__impl_serde $BitFlags:ident: $T:tt) => {
        impl<'de> ::serde::de::Deserialize<'de> for $BitFlags {
            fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> ::std::result::Result<Self, D::Error> {
                let value = <$T>::deserialize(deserializer)?;
                Self::from_bits(value)
                    .ok_or_else(|| {
                        ::serde::de::Error::custom(format!("invalid {} value: {}", stringify!($BitFlags), value))
                    })
            }
        }
    };
    () => {};
}
// }}}

// macro: enum_number {{{
macro_rules! enum_number {
    (
        $(#[$outer:meta])*
        pub enum $name:ident {
            $(
                $(#[$inner:ident $($args:tt)*])*
                $variant:ident = $value:expr,
            )*
        }
    ) => {
        $(#[$outer])*
        #[derive(Clone, Copy)]
        pub enum $name {
            $(
                $(#[$inner $($args)*])*
                $variant = $value,
            )*
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                (*self as u8).fmt(f)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = $name;

                    fn expecting(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.write_str("positive integer")
                    }

                    fn visit_u64<E>(self, value: u64) -> ::std::result::Result<$name, E>
                    where
                        E: ::serde::de::Error,
                    {
                        match value {
                            $( $value => Ok($name::$variant), )*
                            _ => Err(E::custom(format!(
                                "unknown {} value {}",
                                stringify!($name),
                                value
                            ))),
                        }
                    }
                }

                deserializer.deserialize_u64(Visitor)
            }
        }
    };
}
// }}}

// vim: fdm=marker
