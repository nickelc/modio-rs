### v0.4 (not released)

#### Features

* A `Builder` to create a `Modio` client with custom configuration.

```rust
let creds = Credentials::Token("<token>".to_string());
let modio = Modio::builder(creds)
    .host("host")
    .agent("user-agent")
    .build()?;
```

* Proxy support

```rust
let proxy = modio::Proxy::all("http://127.0.0.1:8888")?;
let modio = Modio::builder(creds)
    .proxy(proxy)
    .build()?;
```

* Add optional `rustls-tls` feature to use rustls instead of native-tls.

  if compiled with `default-tls` and `rustls-tls` features then it's possible to choose the backend with `Builder::use_default_tls()` and `Builder::use_rustls_tls()`.

* Add methods to provide streams over entities.

```rust
use modio::filter::prelude::*;
let filter = Fulltext::eq("foobar");

let mods = game.mods().iter(&filter).for_each(|m| {
    // do stuff
});
let stats = game.mods().statistics(&Default::default()).for_each(|stats| {
    // do stuff
});
```

* Add type alias `List<T>` for `ModioListResponse<T>`.

* Add Steam authentication `modio.auth().steam_auth("<auth-ticket>")`.

* Add GOG Galaxy authentication `modio.auth().gog_auth("<auth-ticket>")`.

* Link external accounts `modio.auth().link("email", modio::auth::Service)`.

* `modio::me::Event` with new field `game_id`.

* Validate credentials before sending requests.

* debug & trace log for requests & responses.

#### Breaking Changes

* Rewrite of filtering and sorting.

  ```rust
  // Before
  use modio::filter::{Operator, Order};

  let mut opts = ModsListOptions::new();
  opts.game_id(Operator::In, vec![1, 2]);
  opts.limit(10);
  opts.sort_by(ModsListOptions::POPULAR, Order::Desc);

  // After
  use modio::filter::prelude::*;
  use modio::mods::filters::{GameId, Popular};

  let filter = GameId::_in(vec![1, 2])
      .limit(10)
      .order_by(Popular::desc());
  ```

* Switch from `hyper` to `reqwest`. Type parameter for `Modio` is no longer necessary.

* Drop `failure` crate again and implement std error trait.

* Restrict conversion to `Error` to internal use only.

* `Modio::new` and `Modio::host` return `Result<Modio>`.

* `Modio::custom` removed in flavor of `Builder`.

* User-Agent parameter removed from `Modio::new` and `Modio::host`.

* No longer expose `ModioMessage`.

* Status & visibility mapped as enum.

* Break up event & event types to `modio::me::{Event, EventType}` and `modio::mods::{Event, EventType}`.

* Change `Me::{events, subscriptions, ratings}`, `Mods::{events, statistics}` and `Mod::events` to streams over entities.

### v0.3 (2018-10-04)
* builtin method `Modio::download` for downloading files
  ([c4029f1b](https://github.com/nickelc/modio-rs/commit/c4029f1bd9ba099df582f2c5ce10420d7a85db9c))

#### Breaking Changes
* reworked errors with `failure` crate
  ([0acc1e80](https://github.com/nickelc/modio-rs/commit/0acc1e807ef5de36950604d3d15e7ef86ea88027))

### v0.2.2 (2018-09-20)
* add missing `Mod::stats` property
  ([0af0580b](https://github.com/nickelc/modio-rs/commit/0af0580b9a588024fa38ca60ad419fc499321574))

* update dev dependencies to fix build issues with openssl
  ([41a143e5](https://github.com/nickelc/modio-rs/commit/41a143e54cca35c26517810a3ceecc9aa45a9968))

* new method to add custom filters to list options
  ([a81771c4](https://github.com/nickelc/modio-rs/commit/a81771c4902448d45379eedc4a98aa5f24394827))

### v0.2.1 (2018-09-10)
* use the new endpoint `/me/ratings` to list the submitted mod ratings
  ([09117df5](https://github.com/nickelc/modio-rs/commit/09117df59e6f9a9de2fc104fc458b7f99d5740a8))

* new property `total` for `ModioListResponse` added
  ([f2d84642](https://github.com/nickelc/modio-rs/commit/f2d84642a09159203d7e11ceb6c8cf0cf7414a37))

* new read-only property `Mod::description_plaintext`
  ([743b5c5c](https://github.com/nickelc/modio-rs/commit/743b5c5cbfbfdc16038c76c161e6b8222688ab95))

* fixed query string separator
  ([fa90195c](https://github.com/nickelc/modio-rs/commit/fa90195cab717e27a5a7912f781c2dd8cc350af8))

### v0.2.0 (2018-08-09)

#### Breaking Changes

* `Mod::rating_summary` is gone.
  Replaced with the new statistics endpoints `Mods::statistics` and `ModRef::statistics`.

  ([33388dd3](https://github.com/nickelc/modio-rs/commit/33388dd3686ad8056f92444176ea7b0df6c497b2))
