<a href="https://mod.io"><img src="https://github.com/nickelc/modio-rs/raw/master/header.png" alt="mod.io" width="320"/></a>

[![Crates.io][crates-badge]][crates-url]
[![Released API docs][docs-badge]][docs-url]
[![Master API docs][master-docs-badge]][master-docs-url]
![Rust version][rust-version]
![License][license-badge]
[![Workflow Status][workflow-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/modio.svg
[crates-url]: https://crates.io/crates/modio
[docs-badge]: https://docs.rs/modio/badge.svg
[docs-url]: https://docs.rs/modio
[license-badge]: https://img.shields.io/crates/l/modio.svg
[master-docs-badge]: https://img.shields.io/badge/docs-master-green.svg
[master-docs-url]: https://nickelc.github.io/modio-rs/master/
[workflow-badge]: https://github.com/nickelc/modio-rs/workflows/ci/badge.svg
[actions-url]: https://github.com/nickelc/modio-rs/actions
[rust-version]: https://img.shields.io/badge/rust-1.83.0%2B-lightgrey.svg?logo=rust

`modio` provides a set of building blocks for interacting with the [mod.io](https://mod.io) API.

The client uses asynchronous I/O, backed by the `futures` and `tokio` crates, and requires both to be used alongside.

## mod.io
[mod.io](https://mod.io) is a drop-in modding solution from the founders of [ModDB.com](https://www.moddb.com),
that facilitates the upload, search, browsing, downloading and trading of mods in-game.

## Usage

To use `modio`, execute `cargo add modio`.

### Basic Setup
```rust
use std::env;
use modio::Client;

#[tokio::main]
async fn main() -> Result<(), Box<std::error::Error>> {
    let client = Client::builder(env::var("MODIO_API_KEY")?)
        .token(env::var("MODIO_TOKEN")?)
        .build()?;

    // create some tasks and execute them
    // let result = task.await?;
    Ok(())
}
```

### Authentication
```rust
// Request a security code be sent to the email address.
client.request_code("john@example.com").await?;

// Wait for the 5-digit security code
let response = client.request_token("QWERT").await?;
let token = response.data().await?;

// Create an endpoint with the new token
let client = client.with_token(token.value);
```
See [full example](examples/auth.rs).

### Games
```rust
use modio::request::filter::prelude::*;

// List games with filter `name_id = "0ad"`
let response = client.get_games().filter(NameId::eq("0ad")).await?;
let list = response.data().await?;
```

### Mods
```rust
use modio::request::filter::prelude::*;

// List all mods for 0 A.D.
let response = client.get_mods(Id::new(5)).await?;
let mods = response.data().await?;

// Get the details of the `balancing-mod` mod
let response = client.get_mod(Id::new(5), Id::new(110)).await?;
let balancing_mod = response.data().await?;
```

### Download
```rust
use modio::util::download::{Download, DownloadAction, ResolvePolicy};

// Download the primary file of a mod.
let action = DownloadAction::Primary {
    game_id: Id::new(5),
    mod_id: Id::new(19),
};
modio
    .download(action)
    .save_to_file("mod.zip")
    .await?;

// Download the specific file of a mod.
let action = DownloadAction::File {
    game_id: Id::new(5),
    mod_id: Id::new(19),
    file_id: Id::new(101),
};
modio
    .download(action)
    .save_to_file("mod.zip")
    .await?;

// Download the specific version of a mod.
// if multiple files are found then the latest file is downloaded.
// Set policy to `ResolvePolicy::Fail` to return with `modio::util::download::ErrorKind::MultipleFilesFound`
// as error kind.
let action = DownloadAction::Version {
    game_id: Id::new(5),
    mod_id: Id::new(19),
    version: "0.1".to_string(),
    policy: ResolvePolicy::Latest,
};
let mut chunked = client.download(action).chunked().await?;
while let Some(chunk) = chunked.data().await {
    let chunk = chunk?;
    println!("bytes: {:?}", chunk);
}
```

### Examples

See [examples directory](examples/) for some getting started examples.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you,
as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
