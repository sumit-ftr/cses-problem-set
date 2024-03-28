use reqwest;
use scrape::Config;
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config::new(std::env::args()).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });
    let mut req = reqwest::get(cfg.url).await?;
    println!("{req:?}");
    Ok(())
}
