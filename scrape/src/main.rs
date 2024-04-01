use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cfg = scrape::Config::new(std::env::args()).await?;
    cfg.get_fastest().await?;
    cfg.create_json().await?;
    cfg.write_all_files().await?;
    Ok(())
}
