use std::fmt;

macro_rules! filter_options {
    (
        $(#[$outer:meta])*
        pub struct $FilterOptions:ident {
            Filters
            $(
                $(#[$filter_inner:ident $($filter_args:tt)*])*
                - $filter_fn:ident = $filter_name:expr;
            )+
            Sort
            $(
                $(#[$sort_inner:ident $($sort_args:tt)*])*
                - $sort_ident:ident = $sort_name:expr;
            )+
        }
    ) => {
        $(#[$outer])*
        pub struct $FilterOptions {
            filters: ::std::collections::BTreeMap<String, ::filter::Filter>,
            sorting: Option<String>,
            limit: Option<usize>,
            offset: Option<usize>,
        }

        impl $FilterOptions {
            $(
                $(#[$sort_inner $($sort_args)*])*
                pub const $sort_ident: ::filter::SortField = ::filter::SortField($sort_name);
            )+

            pub fn new() -> Self {
                Default::default()
            }

            pub fn fulltext<T, V>(&mut self, value: V) -> &mut Self
            where
                T: ::std::fmt::Display,
                V: Into<::filter::OneOrMany<T>>,
            {
                let f = ::filter::Filter::new("_q", None, value);
                self.filters.insert(f.name(), f);
                self
            }

            $(
                $(#[$filter_inner $($filter_args)*])*
                pub fn $filter_fn<T, V>(&mut self, op: ::filter::Operator, value: V) -> &mut Self
                where
                    T: ::std::fmt::Display,
                    V: Into<::filter::OneOrMany<T>>,
                {
                    let f = ::filter::Filter::new($filter_name, Some(op), value);
                    self.filters.insert(f.name(), f);
                    self
                }
            )+

            pub fn sort_by(&mut self, field: ::filter::SortField, order: ::filter::Order) -> &mut Self {
                self.sorting = match order {
                    ::filter::Order::Asc => Some(field.to_string()),
                    ::filter::Order::Desc => Some(format!("-{}", field)),
                };
                self
            }

            pub fn limit<T: Into<Option<usize>>>(&mut self, limit: T) -> &mut Self {
                self.limit = limit.into();
                self
            }

            pub fn offset<T: Into<Option<usize>>>(&mut self, offset: T) -> &mut Self {
                self.offset = offset.into();
                self
            }
        }

        impl Default for $FilterOptions {
            fn default() -> Self {
                Self {
                    filters: Default::default(),
                    sorting: None,
                    limit: None,
                    offset: None,
                }
            }
        }

        impl QueryParams for $FilterOptions {
            fn to_query_params(&self) -> String {
                ::url::form_urlencoded::Serializer::new(String::new())
                    .extend_pairs(self.filters.values().map(|f| (f.name(), f.value())))
                    .extend_pairs(self.sorting.iter().map(|s| ("_sort", s)))
                    .extend_pairs(self.offset.iter().map(|o| ("_offset", o.to_string())))
                    .extend_pairs(self.limit.iter().map(|l| ("_limit", l.to_string())))
                    .finish()
            }
        }
    };
}

#[derive(Debug)]
pub struct Filter {
    name: String,
    suffix: Option<Operator>,
    value: OneOrMany<String>,
}

impl Filter {
    pub(crate) fn new<S, T, V>(name: S, suffix: Option<Operator>, value: V) -> Self
    where
        S: Into<String>,
        T: fmt::Display,
        V: Into<OneOrMany<T>>,
    {
        Self {
            name: name.into(),
            suffix,
            value: value.into().to_string(),
        }
    }

    pub fn name(&self) -> String {
        let suffix = self.suffix
            .map(|s| s.to_string())
            .unwrap_or_else(String::new);
        format!("{}{}", self.name, suffix)
    }

    pub fn operator(&self) -> Option<&Operator> {
        self.suffix.as_ref()
    }

    pub fn value(&self) -> String {
        match self.value {
            OneOrMany::One(ref v) => v.to_string(),
            OneOrMany::Many(ref v) => v.iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(",")
                .to_owned(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Operator {
    Equals,
    Not,
    Like,
    NotLike,
    In,
    NotIn,
    Min,
    Max,
    SmallerThan,
    GreaterThan,
    BitwiseAnd,
}

impl fmt::Display for Operator {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operator::Equals => "",
            Operator::Not => "-not",
            Operator::Like => "-lk",
            Operator::NotLike => "-not-lk",
            Operator::In => "-in",
            Operator::NotIn => "-not-in",
            Operator::Min => "-min",
            Operator::Max => "-max",
            Operator::SmallerThan => "-st",
            Operator::GreaterThan => "-gt",
            Operator::BitwiseAnd => "-bitwise-and",
        }.fmt(fmt)
    }
}

#[derive(Debug)]
pub enum OneOrMany<T>
where
    T: fmt::Display,
{
    One(T),
    Many(Vec<T>),
}

impl<T: fmt::Display> OneOrMany<T> {
    fn to_string(&self) -> OneOrMany<String> {
        match *self {
            OneOrMany::One(ref s) => OneOrMany::One(s.to_string()),
            OneOrMany::Many(ref s) => {
                OneOrMany::Many(s.iter().map(ToString::to_string).collect::<Vec<_>>())
            }
        }
    }
}

impl<T: fmt::Display> From<T> for OneOrMany<T> {
    fn from(from: T) -> OneOrMany<T> {
        OneOrMany::One(from)
    }
}

impl<T: fmt::Display> From<Vec<T>> for OneOrMany<T> {
    fn from(from: Vec<T>) -> OneOrMany<T> {
        OneOrMany::Many(from)
    }
}

pub struct SortField(pub(crate) &'static str);

impl fmt::Display for SortField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(fmt)
    }
}

pub enum Order {
    Asc,
    Desc,
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_filter {
        ($fn:ident, $name:expr, $value:expr,expected: $name2:expr, $value2:expr) => {
            #[test]
            fn $fn() {
                let f = Filter::new($name, None, $value);
                assert_eq!(f.name(), $name2);
                assert_eq!(f.value(), $value2);
            }
        };
        ($fn:ident, $op:ident, $name:expr, $value:expr,expected: $name2:expr, $value2:expr) => {
            #[test]
            fn $fn() {
                let f = Filter::new($name, Some(Operator::$op), $value);
                assert_eq!(f.name(), $name2);
                assert_eq!(f.value(), $value2);
            }
        };
    }

    test_filter!(fulltext, "_q", "hello", expected: "_q", "hello");
    test_filter!(equals, Equals, "id", vec!["1", "2"], expected: "id", "1,2");
    test_filter!(not, Not, "id", vec!["1", "2"], expected: "id-not", "1,2");
    test_filter!(like, Like, "id", vec!["1", "2"], expected: "id-lk", "1,2");
    test_filter!(not_like, NotLike, "id", vec!["1", "2"], expected: "id-not-lk", "1,2");
    test_filter!(in_, In, "id", vec!["1", "2"], expected: "id-in", "1,2");
    test_filter!(not_in, NotIn, "id", vec!["1", "2"], expected: "id-not-in", "1,2");
    test_filter!(min, Min, "id", vec!["1", "2"], expected: "id-min", "1,2");
    test_filter!(max, Max, "id", vec!["1", "2"], expected: "id-max", "1,2");
    test_filter!(smaller_than, SmallerThan, "id", vec!["1", "2"], expected: "id-st", "1,2");
    test_filter!(greater_than, GreaterThan, "id", vec!["1", "2"], expected: "id-gt", "1,2");
    test_filter!(bitwise_and, BitwiseAnd, "id", vec!["1", "2"], expected: "id-bitwise-and", "1,2");
}
