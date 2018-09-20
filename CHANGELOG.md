### v0.2.2
* add missing `Mod::stats` property
  ([0af0580b](https://github.com/nickelc/modio-rs/commit/0af0580b9a588024fa38ca60ad419fc499321574))

* update dev dependencies to fix build issues with openssl
  ([41a143e5](https://github.com/nickelc/modio-rs/commit/41a143e54cca35c26517810a3ceecc9aa45a9968))

* new method to add custom filters to list options
  ([a81771c4](https://github.com/nickelc/modio-rs/commit/a81771c4902448d45379eedc4a98aa5f24394827))

### v0.2.1
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
