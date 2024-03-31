use reqwest::{self, cookie::Jar, Client, Url};
use scraper::{Html, Selector};
use serde_json;
use std::collections::BTreeMap;
use std::error::Error;
use tokio::{self, fs::File, io::AsyncWriteExt};

pub struct Config {
    url: String,
    pno: usize, // problem number
    rng: usize, // range of web pages of the problem
    client: Client,
    top_rust: BTreeMap<(u8, u16), String>,
    top_time: BTreeMap<(u8, u16), String>,
}

impl Config {
    pub async fn new(mut a: std::env::Args) -> Result<Self, Box<dyn Error>> {
        a.next();
        let (pno, mut url) = match a.next() {
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
        let mut top_rust = BTreeMap::<(u8, u16), String>::new();
        let mut top_time = BTreeMap::<(u8, u16), String>::new();
        let rng = Self::get_range(&client, &url, &mut top_rust, &mut top_time).await?;
        url.push_str("12/");

        Ok(Self {
            url,
            pno,
            rng,
            client,
            top_rust,
            top_time,
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
            .form(&std::collections::HashMap::from([
                ("csrf_token", csrf_token),
                ("nick", &nick[..nick.len() - 1]),
                ("pass", &pass[..pass.len() - 1]),
            ]))
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_range(
        client: &Client,
        url: &str,
        top_rust: &mut BTreeMap<(u8, u16), String>,
        top_time: &mut BTreeMap<(u8, u16), String>,
    ) -> Result<usize, Box<dyn Error>> {
        let res_body = client.get(url).send().await?.text().await?;
        let fragment = Html::parse_fragment(&res_body);

        // updating the BTreeMaps
        let selector = Selector::parse("td")?;
        let mut submissions = fragment.select(&selector);
        while let Some(_) = submissions.next() {
            submissions.next();
            let lang = submissions.next().unwrap().inner_html();
            let time = submissions.next().unwrap().inner_html();
            let chr_cnt = submissions.next().unwrap().inner_html();
            let endpoint = Html::parse_fragment(&submissions.next().unwrap().inner_html());

            let time = time[..time.len() - 2].parse::<f64>().unwrap() as u8;
            let len = chr_cnt[..chr_cnt.len() - 4].parse::<usize>().unwrap() as u16;
            let value = format!(
                "https://cses.fi{}",
                endpoint
                    .select(&Selector::parse("a")?)
                    .next()
                    .unwrap()
                    .attr("href")
                    .unwrap()
            );

            if lang == "Rust" {
                top_rust.insert((time, len), value);
            } else {
                top_time.insert((time, len), value);
            }
        }

        let rng = fragment
            .select(&Selector::parse("a")?)
            .nth(17)
            .unwrap()
            .inner_html()
            .parse::<usize>()
            .unwrap();

        Ok(rng)
    }

    pub async fn get_fastest(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub async fn get_fastest_by_page(&mut self) -> Result<(), Box<dyn Error>> {
        // while let Some(val) = it.next() {
        //     println!("{:#?}", val.html());
        // }
        Ok(())
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

    pub async fn write_all_files(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
