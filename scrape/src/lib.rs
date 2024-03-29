use reqwest::{self, cookie::Jar, Client, Url};
use scraper::{Html, Selector};
use serde_json;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::{self};
use tokio::{self, fs::File, io::AsyncWriteExt};

pub struct Config {
    url: String,
    pno: usize, // problem number
    rng: usize, // range of web pages of the problem
    client: Client,
    top_rust: BTreeMap<usize, String>,
    top_time: BTreeMap<usize, String>,
}

impl Config {
    pub async fn new(mut a: std::env::Args) -> Result<Self, Box<dyn Error>> {
        a.next();
        let (pno, url) = match a.next() {
            Some(num) => {
                let num = num.parse::<usize>()?;
                (num, format!("https://cses.fi/problemset/hack/{num}/list/"))
            }
            None => return Err(Box::<dyn Error>::from("Error: Problem Number Not Found")),
        };

        let jar = Jar::default();
        jar.add_cookie_str(
            &tokio::fs::read_to_string("userdata/cookie").await?,
            &url.parse::<Url>().unwrap(),
        );
        let client = Client::builder()
            .cookie_store(true)
            .cookie_provider(std::sync::Arc::new(jar))
            .build()?;

        // let res = client.get(&url).send().await?.text().await?;
        println!("{client:#?}\n");
        let req = client.get("https://cses.fi/login").basic_auth(
            tokio::fs::read_to_string("userdata/username").await?,
            Some(tokio::fs::read_to_string("userdata/password").await?),
        );
        println!("{req:#?}\n");
        let res = req.send().await?;
        println!("{res:#?}\n");
        let res = res.text().await?;

        Ok(Self {
            url,
            pno,
            rng: Self::get_range(&res),
            client,
            top_rust: BTreeMap::new(),
            top_time: BTreeMap::new(),
        })
    }

    pub fn get_range(r: &String) -> usize {
        println!("{:#?}", r);
        0
    }

    pub fn get_fastest(&mut self) {
        // key calculation: execution_time * 100 * no_of_chars
    }

    pub async fn create_json(&self) -> Result<(), Box<dyn Error>> {
        // this method is computed separately because in case something happens
        // and the user couldn't able to get the solutions, in that case the user
        // can get all the solutions without scraping all the solution webpages
        fs::create_dir_all(format!("solutions/{}/", self.pno))?;
        let mut ftop_rust = File::create(&format!("solutions/{}/top_rust.json", self.pno)).await?;
        let mut ftop_time = File::create(&format!("solutions/{}/top_time.json", self.pno)).await?;
        ftop_rust
            .write_all(serde_json::to_string(&self.top_rust).unwrap().as_bytes())
            .await?;
        ftop_time
            .write_all(serde_json::to_string(&self.top_time).unwrap().as_bytes())
            .await?;
        Ok(())
    }

    pub fn write_all_files(&self) {}
}
