### v0.13.1 (2025-07-06)

* Add new credit options for mods.
* Add missing community options when adding & editing mods.
* Add new platform status variant.

### v0.13.0 (2025-06-06)

* Increase size of community options for the new undocumented flags.

### v0.12.1 (2025-01-31)

* Fix external links to mod.io's REST API docs.
* Add new dependency option for games.
* Add new community options for games and deprecated renamed flags.
* Add new API access options flags.

### v0.12.0 (2025-01-18)

* Implement `TryFrom<i64>` for `Id<T>` type.
* Introduce `Timestamp` newtype the Unix timestamp fields.

### v0.11.0 (2024-06-04)

* Remove `ALL` constants from bitflags, use the `all()` method instead.
* Add new community options for games and mod objects.

### v0.10.1 (2024-04-11)

* Fix wrong API endpoint url for the security code exchange

### v0.10.0 (2024-03-21)

* Update `reqwest` to 0.12
* Implement `From` trait for newtype integer enums

### v0.9.1 (2023-11-12)

* Implement `std::str::FromStr` for `Id<T>`.

### v0.9.0 (2023-11-09)

* Add method to retrieve the content length for the download request.\
  The download request is now started when calling `Modio::download(action).await?`.
* Add support for reordering mod media files & links.
* Add support for renaming tags.
* Add support for revoking the current access token.
* Add new `locked` field to game's platform object.
* Expose the returned API error from response as getter.
* Use the `retry-after` http header for rate limit checking.
* Map validation errors as a Vec of tupled strings instead of HashMap.
* Add new `dependencies` field to indicate if a mod has dependencies.
* Remove the `service` parameter from the terms endpoint.
* Remove unsupported authentication endpoints for GOG and itch.io.
* Move the `AccessToken` struct into the `types::auth` module.
* Introduce type-safe ID type for resource (games, mods, files, etc.).\
  The `Id<T>` newtype wraps each resource ID as non-zero u64.
* Change string enums to newtypes with associated constants.
* Make the `types` module public.
* Remove deprecated & unstable items:
    * `virustotal_hash` field from mods.
    * `revenue_options` field from games.
    * deprecated flags from `community_options`.
    * `MonetisationOptions` from games and mods.

### v0.8.3 (2023-10-02)

* Move `serde_test` to dev dependencies.

### v0.8.2 (2023-09-22)

* Add workaround for renamed `monetisation_options` field.
* Increase size of community options for games.

### v0.8.1 (2023-09-21)

* Add workaround for missing `monetisation_options` field for mods.
* Export missing bitflags options for games and mods.

### v0.8.0 (2023-06-26)

* Update bitflags to allow unsupported flags.
* Change the game's maturity options into bitflags.
* Change the virus scan status & result fields into enums.
* Add community options for mods.
* Add new monetisation options and deprecate the revenue options.
* Change number enums to newtypes with associated constants.

```rust
// Before
pub enum Visibility {
    Hidden,
    Public,
    Unknown(u8),
}

impl From<Visibility> for u8 {
    fn from(vis: Visibility) -> Self {
        match vis {
            Visibility::Hidden => 0,
            Visibility::Public => 1,
            Visibility::Unknown(value) => value,
        }
    }
}

// After
pub struct Visibility(u8);

impl Visibility {
    pub const HIDDEN: Self = Self(0);
    pub const PUBLIC: Self = Self(1);

    fn new(raw_value: u8) -> Self { Self(raw_value) }
    fn get(self) -> u8 { self.0 }
}
```

### v0.7.4 (2023-06-12)

* Export the `VirusScan` struct for mod files.
* Deprecate virustotal hash string for mod files.
* Add uncompressed filesize attribute for mod files.

### v0.7.3 (2023-06-09)

- Add new `Source` platform.

### v0.7.2 (2023-03-17)

- Bump MSRV to 1.60
- Add new community options for games
- Add missing `Clone` trait impls

### v0.7.1 (2022-11-21)

* Make the endpoint & reference types clonable.

### v0.7.0 (2022-09-01)

* Add support for muting users.

* Update/add fields to several structs.
  `Game`: +stats +theme +other\_urls
  `TagOption`: +locked
  Mod `Statistics`: +downloads\_today
  Mod `Comment`: +resource\_id -mod\_id -karma\_guest

* Remove the `submitted_by` field from the game object.

* Edit game endpoint is removed.

* Add/edit/delete endpoints for team members are removed.

* Add support for the new `platforms` fields of game, mod and modfile structs.

* Update supported target platforms & portals. (Platform: +Oculus -Wii, Portal: +Facebook +Discord)

* Rename `EventType::Other` enum variants to `Unknown`.

* Preserve values for unknown enum variants.

### v0.6.3 (2022-08-08)

* Fix missing feature for `tokio-util`.

* Add EGS as a service for the terms endpoint.

* Add support for the `X-Modio-Platform`/`X-Modio-Portal` headers.

* Add new `tag_count` field to `TagOption`.

* Add Google auth endpoint.

* Allow mod rating to be reset.

### v0.6.2 (2021-02-13)

* Add support for the new terms endpoint.

* Add error variant for the case when the acceptance of Terms of Use is required.

* Don't ignore deserialization errors for metadata kvp data.

### v0.6.1 (2021-01-28)

* Make cloning of the client cheaper.

* Fix the deserialization of mod event types for unknown events.

* Improve serde's error message for untagged enums

### v0.6.0 (2021-01-05)

* Update to tokio 1.0 and reqwest 0.11

### v0.5.2 (2020-11-24)

* Update pin-project-lite to 0.2.

### v0.5.1 (2020-11-10)

* Improve the crate description.

* Rearrange the readme badges + MSRV badge.

### v0.5 (2020-11-02)

* Switch to `async/await`, `std::future` and `tokio 0.2`.

* Rework the `Error` type, remove several `Error::is_*` methods. ([a230d3c])

* Replace `DownloadAction::Url` with `DownloadAction::FileObj`. ([1fd5ff6], [8fbd8d1])

* Introduce `Downloader` with `save_to_file(path)`, `bytes()` and `stream()` methods. ([3ba706a], [b4b7a87])

```rust
// Before
let action = modio::DownloadAction::Primary {
    game_id: 123,
    mod_id: 45,
};

let file = std::file::File::create("mod.zip")?;
let (len, out) = rt.block_on(modio.download(action, out))?;

// After
modio.download(action).save_to_file("mod.zip").await?;

let bytes: Bytes = modio.download(action).bytes().await?;

let stream = Box::pin(modio.download(action).stream());
while let Some(bytes) = stream.try_next().await? {
    // process(bytes)
}
```

* Remove `Users::get|list|iter` methods. The `/users` endpoints are no longer supported. ([1c547aa])

* Replace list & iter methods with `search(filter)` returning the `Query<T>` type which implements
  various methods to load search results. ([ebf5374], [f8a35de])

```rust
// Before
let stream = modio.games().iter(&filter);

let first_page: modio::List<Game> = rt.block_on(modio.games().list(&filter));

// After
let stream = modio.games().search(filter).iter().await?;

let first: Option<Game> = modio.games().search(filter).first().await?;
let first_page: Vec<Game> = modio.games().search(filter).first_page().await?;

let list: Vec<Game> = modio.games().search(filter).collect().await?;

// stream of `modio::Page<Game>`
let stream = modio.games().search(filter).paged().await?;
```

* Add Oculus, itch.io, Xbox Live, Discord & Switch authentication.
  ([5d46974], [2315236], [96fdc07], [013f43d], [38698cc])

* Add expiry date of the access token. ([9445c3c])

* Remove all deprecated code. ([c3032af])

* Update `url` to v2.0.

* Use `tracing` instead of `log`. ([0a1c2e4])

* New endpoints for adding & editing comments and game stats. ([1062775], [633cf28])

* New event type variants for added/deleted comments and unsupported with `Other(String)`.
  ([8a85576], [d636096])

* Add modio's new `error_ref` error code. ([24b7c33])

[a230d3c]: https://github.com/nickelc/modio-rs/commit/a230d3c790e2eb3d1d03160f3e3f1219c2f4fc34
[1fd5ff6]: https://github.com/nickelc/modio-rs/commit/1fd5ff67597e57975684a735b88a949f44d775bc
[8fbd8d1]: https://github.com/nickelc/modio-rs/commit/8fbd8d1017738dcaacf8a807f43c0e6640f93552
[3ba706a]: https://github.com/nickelc/modio-rs/commit/3ba706a3020576f425d2fc75122ee9d459f55972
[b4b7a87]: https://github.com/nickelc/modio-rs/commit/b4b7a8709e4c9ecc70c8ad98aa4849ca7f187391
[1c547aa]: https://github.com/nickelc/modio-rs/commit/1c547aa1b9751d6bfb4185d13f685df5136fd052
[ebf5374]: https://github.com/nickelc/modio-rs/commit/ebf5374e1396c3b502e858d973d63396e2d6b1dd
[f8a35de]: https://github.com/nickelc/modio-rs/commit/f8a35de3906542bbb16b2c477c34e4e4e04cee0b

[5d46974]: https://github.com/nickelc/modio-rs/commit/5d469749265a58f73eba140de7fccf90e2efc03d
[2315236]: https://github.com/nickelc/modio-rs/commit/2315236c5fa3909004586f5cc164dfe78f0414b5
[96fdc07]: https://github.com/nickelc/modio-rs/commit/96fdc0722b2cd944b79a6ae19e69d52e402477e3
[013f43d]: https://github.com/nickelc/modio-rs/commit/013f43d46b1d21c2cbd4b3b1011721c1daeeb0d0
[38698cc]: https://github.com/nickelc/modio-rs/commit/38698cc98baab8def4b780cd4fe9104800f3143f

[9445c3c]: https://github.com/nickelc/modio-rs/commit/9445c3c6e9efcf5d50f730c41c20288689264aeb
[c3032af]: https://github.com/nickelc/modio-rs/commit/c3032af3b44acaa88b331c55bffc43324289030a
[0a1c2e4]: https://github.com/nickelc/modio-rs/commit/0a1c2e47d2ab0ae71b340100b5f505aed4f46caa
[1062775]: https://github.com/nickelc/modio-rs/commit/10627752df9c435cea9cdda239b5f649aa9d1598
[633cf28]: https://github.com/nickelc/modio-rs/commit/633cf2835f4f24442258fb655c64633b1daa87d1
[8a85576]: https://github.com/nickelc/modio-rs/commit/8a8557610287cb24c8ebd24b3581fea585f78968
[d636096]: https://github.com/nickelc/modio-rs/commit/d6360962696f757400fb118436285b75dedf946e
[24b7c33]: https://github.com/nickelc/modio-rs/commit/24b7c33b887ab051b7b2fc63ce886ebd2c155e9c

### v0.4.2 (not released)

* New `Error::is_authentication` accessor

* Fix typo `EditDependenciesOptions`

* Replace `ModioResult` with deprecated type alias for `EntityResult`.

* Replace `ModioListResponse` with deprecated type alias for `List`.

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
