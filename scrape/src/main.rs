use scrape::Config;
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut cfg = Config::new(std::env::args()).await.unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });
    cfg.get_fastest();
    cfg.create_json()?;
    cfg.write_all_files();
    Ok(())
}
