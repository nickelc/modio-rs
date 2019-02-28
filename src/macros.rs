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
