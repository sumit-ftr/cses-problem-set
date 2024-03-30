use reqwest::{self, cookie::Jar, Client, Url};
use scraper::{Html, Selector};
use serde_json;
use std::collections::{BTreeMap, HashMap};
use std::error::Error;
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

        let cookie = tokio::fs::read_to_string("userdata/cookie").await?;
        let jar = Jar::default();
        jar.add_cookie_str(&cookie[..cookie.len() - 1], &url.parse::<Url>().unwrap());
        let client = Client::builder()
            .cookie_store(true)
            .cookie_provider(std::sync::Arc::new(jar))
            .build()?;

        Self::authenticate(&client).await?;
        let rng = Self::get_range(&client, &url).await?;

        Ok(Self {
            url,
            pno,
            rng,
            client,
            top_rust: BTreeMap::new(),
            top_time: BTreeMap::new(),
        })
    }

    pub async fn authenticate(client: &Client) -> Result<(), Box<dyn Error>> {
        let login_url = "https://cses.fi/login";

        // getting the login form to get the csrf token
        let login_form = client.get(login_url).send().await?.text().await?;
        let fragment = Html::parse_fragment(&login_form);
        let csrf_token = fragment
            .select(&Selector::parse(r#"input[name="csrf_token"]"#)?)
            .next()
            .unwrap()
            .attr("value")
            .unwrap();

        // authenticating the user using the csrf token, username & password
        let nick = tokio::fs::read_to_string("userdata/username").await?;
        let pass = tokio::fs::read_to_string("userdata/password").await?;
        client
            .post(login_url)
            .form(&HashMap::from([
                ("csrf_token", csrf_token),
                ("nick", &nick[..nick.len() - 1]),
                ("pass", &pass[..pass.len() - 1]),
            ]))
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_range(client: &Client, url: &str) -> Result<usize, Box<dyn Error>> {
        let res_body = client.get(url).send().await?.text().await?;
        println!("{res_body:?}");
        let fragment = Html::parse_fragment(&res_body);
        let rng = fragment
            .select(&Selector::parse(r#"div[class="pager full-width"]"#)?)
            .next()
            .unwrap();
        println!("{rng:?}");

        Ok(0)
    }

    pub fn get_fastest(&mut self) {
        // key calculation: execution_time * 100 * no_of_chars
    }

    pub async fn create_json(&self) -> Result<(), Box<dyn Error>> {
        // this method is computed separately because in case something happens
        // and the user couldn't able to get the solutions, in that case the user
        // can get all the solutions without scraping all the solution webpages
        tokio::fs::create_dir_all(format!("solutions/{}/", self.pno)).await?;
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
