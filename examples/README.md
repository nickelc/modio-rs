## Examples of using modio

Run examples with `cargo run --example example_name`

### Available examples

* [`auth`](auth.rs) - Request an access token and print the authenticated user. See [Email Authentication Flow](https://docs.mod.io/#email-authentication-flow).

* [`download`](download.rs) - Download the latest modfile for a given mod of a game.

* [`events`](events.rs) - Poll the user events from [`/me/events`](https://docs.mod.io/#get-user-events) every 10 seconds.

* [`mymods`](mymods.rs) - List all mods the *authenticated user* added or is team member of. See [`/me/mods`](https://docs.mod.io/#get-user-mods).
