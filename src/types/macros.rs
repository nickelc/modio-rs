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
        $(#[$outer])*
        #[derive(Copy, Clone, Eq, PartialEq, Deserialize)]
        $vis struct $BitFlags($T);

        bitflags::bitflags! {
            impl $BitFlags: $T {
                $(
                    $(#[$inner $($args)*])*
                    const $Flag = $value;
                )*
            }
        }

        impl std::fmt::Debug for $BitFlags {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                struct Internal($BitFlags);
                impl std::fmt::Debug for Internal {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        bitflags::parser::to_writer(&self.0, f)
                    }
                }
                let mut tuple = f.debug_tuple(stringify!($BitFlags));
                if self.is_empty() {
                    tuple.field(&format_args!("{0:#x}", <$T as bitflags::Bits>::EMPTY));
                } else {
                    tuple.field(&Internal(*self));
                }
                tuple.finish()
            }
        }

        impl ::std::fmt::Display for $BitFlags {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                self.bits().fmt(f)
            }
        }

        bitflags! {
            $($t)*
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
    (
        $(#[$outer:meta])*
        $vis:vis struct $NewtypeEnum:ident<$LENGTH:literal> {
            $(
                $(#[$inner:meta $($args:tt)*])*
                const $Variant:ident = $value:expr;
            )*
        }

        $($t:tt)*
    ) => {
        $(#[$outer])*
        #[derive(Clone, Copy, Eq, PartialEq)]
        $vis struct $NewtypeEnum(crate::types::utils::SmallStr<$LENGTH>);

        impl $NewtypeEnum {
            $(
                $(#[$inner $($args)*])*
                pub const $Variant: Self = Self::from_bytes($value);
            )*

            const fn from_bytes(input: &[u8]) -> Self {
                Self(crate::types::utils::SmallStr::from_bytes(input))
            }

            pub fn as_str(&self) -> &str {
                self.0.as_str()
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

        impl PartialEq<&str> for $NewtypeEnum {
            fn eq(&self, other: &&str) -> bool {
                self.as_str().eq_ignore_ascii_case(other)
            }
        }

        impl PartialEq<$NewtypeEnum> for &str {
            fn eq(&self, other: &$NewtypeEnum) -> bool {
                self.eq_ignore_ascii_case(other.as_str())
            }
        }

        impl PartialEq<str> for $NewtypeEnum {
            fn eq(&self, other: &str) -> bool {
                self.as_str().eq_ignore_ascii_case(other)
            }
        }

        impl PartialEq<$NewtypeEnum> for str {
            fn eq(&self, other: &$NewtypeEnum) -> bool {
                self.eq_ignore_ascii_case(other.as_str())
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
