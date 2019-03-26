macro_rules! future_err {
    ($e:expr) => {
        Box::new(futures::future::err($e))
    };
}

macro_rules! stream_err {
    ($e:expr) => {
        Box::new(futures::stream::once(Err($e)))
    };
}

macro_rules! apikey_required {
    ($m:expr) => {
        if let crate::auth::Credentials::Token(_) = $m.credentials {
            return future_err!(crate::error::apikey_required());
        }
    };
}

macro_rules! token_required {
    ($m:expr) => {
        if let crate::auth::Credentials::ApiKey(_) = $m.credentials {
            return future_err!(crate::error::token_required());
        }
    };
    (s $m:expr) => {
        if let crate::auth::Credentials::ApiKey(_) = $m.credentials {
            return stream_err!(crate::error::token_required());
        }
    };
}

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
