use std::env;
use std::io::{self, Write};

use modio::Client;

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

    let host = env::var("MODIO_HOST").unwrap_or_else(|_| "api.test.mod.io".to_string());

    let api_key = prompt("Enter api key: ")?;
    let email = prompt("Enter email: ")?;

    let client = Client::builder(api_key).host(host).build()?;

    let terms = client.get_terms().await?.data().await?;
    println!("Terms:\n{}\n", terms.plaintext);

    match &*prompt("Accept? [Y/n]: ")? {
        "" | "y" | "Y" => {}
        _ => return Ok(()),
    }

    client.request_code(&email).await?;

    let code = prompt("Enter security code: ").expect("read code");
    let token = client.request_token(&code).await?.data().await?;
    println!("Access token:\n{}", token.value);

    // Consume the endpoint and create an endpoint with new credentials.
    let client = client.with_token(token.value);
    let user = client.get_authenticated_user().await?.data().await?;
    println!("Authenticated user:\n{:#?}", user);

    Ok(())
}
