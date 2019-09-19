macro_rules! option {
    ($(#[$outer:meta])* $name:ident) => {
        option!($(#[$outer])* $name: Into<String>);
    };
    ($(#[$outer:meta])* $name:ident: Into<$T:ty>) => {
        $(#[$outer])*
        pub fn $name<T: Into<$T>>(self, value: T) -> Self {
            Self {
                $name: Some(value.into()),
                ..self
            }
        }
    };
    ($(#[$outer:meta])* $name:ident: $T:ty) => {
        $(#[$outer])*
        pub fn $name(self, value: $T) -> Self {
            Self {
                $name: Some(value),
                ..self
            }
        }
    };
    ($(#[$outer:meta])* $name:ident >> $param:expr) => {
        $(#[$outer])*
        pub fn $name<S: Into<String>>(self, value: S) -> Self {
            let mut params = self.params;
            params.insert($param, value.into());
            Self { params }
        }
    };
    ($(#[$outer:meta])* $name:ident: Into<$T:ty> >> $param:expr) => {
        $(#[$outer])*
        pub fn $name<T: Into<$T>>(self, value: T) -> Self {
            let mut params = self.params;
            params.insert($param, value.into().to_string());
            Self { params }
        }
    };
    ($(#[$outer:meta])* $name:ident: $T:ty >> $param:expr) => {
        $(#[$outer])*
        pub fn $name(self, value: $T) -> Self {
            let mut params = self.params;
            params.insert($param, value.to_string());
            Self { params }
        }
    };
}
