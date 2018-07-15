extern crate modio;
extern crate tokio;

use std::io::{self, Write};
use tokio::runtime::Runtime;

use modio::error::Error;
use modio::{auth::Credentials, Modio};

fn prompt(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn main() -> Result<(), Error> {
    let api_key = prompt("Enter api key: ")?;
    let email = prompt("Enter email: ")?;

    let mut rt = Runtime::new()?;
    let modio = Modio::host(
        "https://api.test.mod.io/v1",
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        Credentials::ApiKey(api_key),
    );

    println!("{:#?}", rt.block_on(modio.auth().request_code(&email))?);

    let code = prompt("Enter security code: ")?;
    let token = rt.block_on(modio.auth().security_code(&code))?;
    println!("Access token:\n{}", token);
    Ok(())
}
