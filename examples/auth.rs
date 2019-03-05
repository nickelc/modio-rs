use std::env;
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
    dotenv::dotenv().ok();

    let host = env::var("MODIO_HOST").unwrap_or_else(|_| "https://api.test.mod.io/v1".to_string());

    let api_key = prompt("Enter api key: ").expect("read api key");
    let email = prompt("Enter email: ").expect("read email");

    let mut rt = Runtime::new().expect("new rt");
    let modio = Modio::host(host, Credentials::ApiKey(api_key))?;

    println!("{:#?}", rt.block_on(modio.auth().request_code(&email))?);

    let code = prompt("Enter security code: ").expect("read code");
    let token = rt.block_on(modio.auth().security_code(&code))?;
    println!("Access token:\n{}", token);

    // Consume the endpoint and create an endpoint with new credentials.
    let modio = modio.with_credentials(token);

    let user = rt.block_on(modio.me().authenticated_user())?;
    println!("Authenticated user:\n{:#?}", user);

    Ok(())
}
