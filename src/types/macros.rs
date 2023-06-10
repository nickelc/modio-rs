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

// macro: newtype_enum {{{
macro_rules! newtype_enum {
    (
        $(#[$outer:meta])*
        $vis:vis struct $NewtypeEnum:ident: $T:ty {
            $(
                $(#[$inner:meta $($args:tt)*])*
                const $Variant:ident = $value:expr;
            )*
        }

        $($t:tt)*
    ) => {
        $(#[$outer])*
        #[derive(Clone, Copy, Eq, PartialEq, Deserialize)]
        $vis struct $NewtypeEnum($T);

        impl $NewtypeEnum {
            $(
                $(#[$inner $($args)*])*
                pub const $Variant: Self = Self($value);
            )*

            /// Create a new value from a raw value.
            pub fn new(raw_value: $T) -> Self {
                Self(raw_value)
            }

            /// Retrieve the raw value.
            pub fn get(self) -> $T {
                self.0
            }
        }

        impl std::fmt::Debug for $NewtypeEnum {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match *self {
                    $(Self::$Variant => f.write_str(concat!(stringify!($NewtypeEnum), "::", stringify!($Variant))),)*
                    _ => f.debug_tuple(stringify!($NewtypeEnum)).field(&self.0).finish(),
                }
            }
        }

        impl std::fmt::Display for $NewtypeEnum {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(&self.0, f)
            }
        }

        newtype_enum! {
            $($t)*
        }
    };
    () => {};
}
// }}}

// vim: fdm=marker
