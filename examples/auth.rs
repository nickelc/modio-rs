use std::env;
use std::io::{self, Write};

use modio::{auth::Credentials, Modio};

fn prompt(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let host = env::var("MODIO_HOST").unwrap_or_else(|_| "https://api.test.mod.io/v1".to_string());

    let api_key = prompt("Enter api key: ")?;
    let email = prompt("Enter email: ")?;

    let modio = Modio::host(host, Credentials::new(api_key))?;

    modio.auth().request_code(&email).await?;

    let code = prompt("Enter security code: ").expect("read code");
    let new_creds = modio.auth().security_code(&code).await?;
    if let Some(token) = &new_creds.token {
        println!("Access token:\n{}", token.value);
    }

    // Consume the endpoint and create an endpoint with new credentials.
    let modio = modio.with_credentials(new_creds);
    let user = modio.user().current().await?;
    println!("Authenticated user:\n{:#?}", user);

    Ok(())
}
