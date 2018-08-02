<a href="https://mod.io"><img src="https://static.mod.io/v1/images/branding/modio-color-dark.svg" alt="mod.io" width="400"/></a>

# modio-rs
![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-brightgreen.svg)
[![Master API docs](https://img.shields.io/badge/docs-master-green.svg)](https://nickelc.github.io/modio-rs/)

`modio` provides a set of building blocks for interacting with the [mod.io](https://mod.io) API.

The client uses asynchronous I/O, backed by the `futures` and `tokio` crates, and requires both to be used alongside.

## mod.io
[mod.io](https://mod.io) is a drop-in modding solution from the founders of [ModDB.com](https://moddb.com),
that facilitates the upload, upload, search, browsing, downloading and trading of mods in-game.

## Usage

To use `modio`, add this to your `Cargo.toml`
```toml
[dependencies]
modio = "0.1"
```

### Basic Setup
```rust
extern crate modio;
extern crate tokio;

use modio::{Credentials, Error, Modio};
use tokio::runtime::Runtime;

fn main() -> Result<(), Error> {
    let mut rt = Runtime::new()?;
    let modio = Modio::new(
        "user-agent-name/1.0",
        Credentials::ApiKey(String::from("user-or-game-apikey")),
    );

    // create some tasks and execute them
    // let result = rt.block_on(task)?;
    Ok(())
}
```

### Authentication
```rust
// Request a security code be sent to the email address.
rt.block_on(modio.auth().request_code("john@example.com"))?;

// Wait for the 5-digit security code
let token = rt.block_on(modio.auth().security_code("QWERT"))?;

// Create an endpoint with the new credentials
let modio = modio.with_credentials(Credentials::Token(token));
```
See [full example](examples/authentication.rs).

### Games
```rust
use modio::filter::Operator;
use modio::games::GamesListOptions;

// List games with filter `name_id = "0ad"`
let task = modio.games().list(
    &GamesListOptions::new()
        .name_id(Operator::Equals, "0ad"),
);

let games = rt.block_on(task)?;
```

### Mods
```rust
// List all mods for 0 A.D.
let mods = rt.block_on(modio.game(5).mods().list(&Default::default))?;

// Get the details of the `balancing-mod` mod
let balancing_mod = rt.block_on(modio.mod_(5, 110).get())?;
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
