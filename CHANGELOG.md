### v0.4.1 (2020-02-05)

* Add new `modio::games::CommunityOptions::DISABLE_SUBSCRIBE` flag. ([bde909fd][bde909fd])

[bde909fd]: https://github.com/nickelc/modio-rs/commit/bde909fdd095210122f095a1d83c3436d381a349

### v0.4 (2019-04-01)

#### Features

* A `Builder` to create a `Modio` client with custom configuration. ([45de8cc6][45de8cc6])

```rust
let creds = Credentials::Token("<token>".to_string());
let modio = Modio::builder(creds)
    .host("host")
    .agent("user-agent")
    .build()?;
```

* Proxy support ([2b12b40a][2b12b40a])

```rust
let proxy = modio::client::Proxy::all("http://127.0.0.1:8888")?;
let modio = Modio::builder(creds)
    .proxy(proxy)
    .build()?;
```

* Add optional `rustls-tls` feature to use rustls instead of native-tls. ([a12b4aa8][a12b4aa8])

  if compiled with `default-tls` and `rustls-tls` features then it's possible to choose the backend with `Builder::use_default_tls()` and `Builder::use_rustls_tls()`.

* Add methods to provide streams over entities. ([39bd3287][39bd3287], [2a47d67c][2a47d67c])

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

* Add Steam authentication `modio.auth().steam_auth("<auth-ticket>")`. ([60072f86][60072f86])

* Add GOG Galaxy authentication `modio.auth().gog_auth("<auth-ticket>")`. ([6e1b1e67][6e1b1e67])

* Link external accounts `modio.auth().link("email", modio::auth::Service)`. ([30b158ab][30b158ab])

* `modio::me::Event` with new field `game_id`.

* Validate credentials before sending requests.

* debug & trace log for requests & responses.

#### Breaking Changes

* Rewrite of filtering and sorting. ([e94c4dcd][e94c4dcd])

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

* Removed builders of all \*Options types and changed the options to be by-value instead of by-ref.
  ([7fe661b6][7fe661b6], [07c3ecb6][07c3ecb6])

  ```rust
  // Before
  let mut builder = EditModOptions::builder();
  if some_val {
      builder.name("foobar");
  }
  let opts = builder.build();
  modio.mod_(34, 101).edit(&opts);

  // After
  let mut opts = EditModOptions::default();
  if some_val {
      opts = opts.name("foobar");
  }
  modio.mod_(34, 101).edit(&opts);
  ```

* `GameRef::edit`, `ModRef::edit` and `FileRef::edit` are now returning `Future<modio::ModioResult<T>>`.
  ([6b31ac4a][6b31ac4a])

* Switch from `hyper` to `reqwest`. Type parameter for `Modio` is no longer necessary.

* Drop `failure` crate again and implement std error trait.

* Restrict conversion to `Error` to internal use only. ([1ac2b471][1ac2b471])

* `Modio::new` and `Modio::host` return `Result<Modio>`.

* `Modio::custom` removed in flavor of `Builder`.

* User-Agent parameter removed from `Modio::new` and `Modio::host`.

* No longer expose `ModioMessage`.

* New ErrorKind for validation errors. ([ca4fe09b][ca4fe09b])

* Map status, visibility and other options as enums and bitfields as `bitflags`.
  ([97a86e8a][97a86e8a], [f2f1acec][f2f1acec])

* Break up event & event types to `modio::me::{Event, EventType}` and `modio::mods::{Event, EventType}`.
  ([57fc4447][57fc4447])

* Change `Me::{events, subscriptions, ratings}`, `Mods::{events, statistics}` and `Mod::events` to streams over entities.
  ([2a47d67c][2a47d67c])

[45de8cc6]: https://github.com/nickelc/modio-rs/commit/45de8cc6f13c15abacbf55d43c956efd2f781950
[2b12b40a]: https://github.com/nickelc/modio-rs/commit/2b12b40afdf87e42460e3a37a3fd69dfc2e8db6b
[a12b4aa8]: https://github.com/nickelc/modio-rs/commit/a12b4aa89c1126dc83100646d8d84dd789bc7f61
[39bd3287]: https://github.com/nickelc/modio-rs/commit/39bd3287b65066c9bfe410f16165b0383d4fa444
[2a47d67c]: https://github.com/nickelc/modio-rs/commit/2a47d67c2a272af8c4e03593e801cb455b121e0e
[60072f86]: https://github.com/nickelc/modio-rs/commit/60072f8672f06f2cea815aa6f4f659d44be974a0
[30b158ab]: https://github.com/nickelc/modio-rs/commit/30b158abedae6b9e71cae66fcdc440f89eafa413
[6e1b1e67]: https://github.com/nickelc/modio-rs/commit/6e1b1e675187c4df6d51972b2bc938353dac7071
[e94c4dcd]: https://github.com/nickelc/modio-rs/commit/e94c4dcdd0a8ef23df338b1945bade4bdb2896a1
[7fe661b6]: https://github.com/nickelc/modio-rs/commit/7fe661b68f50794b40db475993e3cab8acc19dd3
[07c3ecb6]: https://github.com/nickelc/modio-rs/commit/07c3ecb6c9946c64565d8c28c28ccc3a040aed53
[ca4fe09b]: https://github.com/nickelc/modio-rs/commit/ca4fe09b506d9fc393ccf4084879a8e97068eb37
[97a86e8a]: https://github.com/nickelc/modio-rs/commit/97a86e8ad50f3251d1b561fe75e997627fd8e19a
[f2f1acec]: https://github.com/nickelc/modio-rs/commit/f2f1acec4f4c011e60de613d3c86547bc60c019a
[6b31ac4a]: https://github.com/nickelc/modio-rs/commit/6b31ac4abee97521376803f150e1f9f0ce5c8781
[1ac2b471]: https://github.com/nickelc/modio-rs/commit/1ac2b4710373c598c87a9b78e293b68329266c38
[57fc4447]: https://github.com/nickelc/modio-rs/commit/57fc444761499a21ef58ffa6bb81e4ff6f99be1f

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
