use std::env;
use std::io::{self, Write};

use modio::error::Error;
use modio::{auth::Credentials, Modio};

fn prompt(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    env_logger::init();

    let host = env::var("MODIO_HOST").unwrap_or_else(|_| "https://api.test.mod.io/v1".to_string());

    let api_key = prompt("Enter api key: ").expect("read api key");
    let email = prompt("Enter email: ").expect("read email");

    let modio = Modio::host(host, Credentials::ApiKey(api_key))?;

    modio.auth().request_code(&email).await?;

    let code = prompt("Enter security code: ").expect("read code");
    let token = modio.auth().security_code(&code).await?;
    println!("Access token:\n{}", token);

    // Consume the endpoint and create an endpoint with new credentials.
    let modio = modio.with_credentials(token);
    let user = modio.me().authenticated_user().await?;
    println!("Authenticated user:\n{:#?}", user);

    Ok(())
}
