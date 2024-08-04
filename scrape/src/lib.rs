use reqwest::{self, cookie::Jar, Client, Url};
use scraper::{Html, Selector};
use serde;
use serde_json;
use std::{collections::BTreeMap, error::Error, sync::Arc};
use tokio::{self, fs::File, io::AsyncWriteExt};

pub struct Config {
    url: String,             // hacking base url
    rating: usize,           // problem rating
    dirname: Option<String>, // problem number + problem name
    client: Client,
    top_rust: BTreeMap<(u8, u16), String>,
    top_time: BTreeMap<(u8, u16), String>,
}

impl Config {
    pub async fn new(mut a: std::env::Args) -> Result<Self, Box<dyn Error>> {
        a.next();
        // matching first argument (problem rating)
        let (rating, url) = match a.next() {
            Some(num) => {
                let rating = num.parse::<usize>()?;
                (
                    rating,
                    format!("https://cses.fi/problemset/queue/{rating}/"),
                )
            }
            None => return Err(Box::<dyn Error>::from("Error: Problem Number Not Found")),
        };

        // setting up client with the cookie
        let cookie = tokio::fs::read_to_string("userdata/cookie").await?;
        let jar = Jar::default();
        jar.add_cookie_str(&cookie[..cookie.len() - 1], &url.parse::<Url>().unwrap());
        let client = Client::builder()
            .cookie_store(true)
            .cookie_provider(std::sync::Arc::new(jar))
            .build()?;

        Ok(Self {
            url,
            dirname: None,
            rating,
            client,
            top_rust: BTreeMap::<(u8, u16), String>::new(),
            top_time: BTreeMap::<(u8, u16), String>::new(),
        })
    }

    pub async fn authenticate(&mut self) -> Result<(), Box<dyn Error>> {
        let login_url = "https://cses.fi/login";

        // getting the login form to get the csrf token
        let login_form = self.client.get(login_url).send().await?.text().await?;
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
        self.client
            .post(login_url)
            .form(&std::collections::HashMap::from([
                ("csrf_token", csrf_token),
                ("nick", &nick[..nick.len() - 1]),
                ("pass", &pass[..pass.len() - 1]),
            ]))
            .send()
            .await?;

        println!("Authentication complete!");

        Ok(())
    }

    pub fn get_fastest_by_page(
        res_body: &String,
        top_rust: &mut BTreeMap<(u8, u16), String>,
        top_time: &mut BTreeMap<(u8, u16), String>,
    ) -> Result<(), Box<dyn Error>> {
        let document = Html::parse_document(res_body);

        // updating the BTreeMaps
        let selector = Selector::parse("td").unwrap();
        let mut submissions = document.select(&selector);
        while let Some(_) = submissions.next() {
            submissions.next();
            // finding values from the html
            let lang = submissions.next().unwrap().inner_html();
            let time = submissions.next().unwrap().inner_html();
            let chr_cnt = submissions.next().unwrap().inner_html();
            let endpoint = Html::parse_document(&submissions.next().unwrap().inner_html());
            // computing the key & values of the BTreeMaps
            let time = (time[..time.len() - 2].parse::<f64>().unwrap() * 100.0) as u8;
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
            // inserting values into BTreeMaps
            if lang == "Rust" {
                top_rust.insert((time, len), value);
            } else {
                if top_time.len() > 63 {
                    top_time.pop_last();
                }
                top_time.insert((time, len), value);
            }
        }
        Ok(())
    }

    pub async fn get_fastest(&mut self) -> Result<(), Box<dyn Error>> {
        let res_body = self
            .client
            .get(format!("{}1", self.url))
            .send()
            .await?
            .text()
            .await?;

        // takes the response body and gets the fastest solutions
        Self::get_fastest_by_page(&res_body, &mut self.top_rust, &mut self.top_time)?;
        println!("parsed 1st page successfully");

        let document = Html::parse_document(&res_body);
        self.dirname = Some(format!(
            "{}-{}",
            self.rating,
            document
                .select(&Selector::parse("h1").unwrap())
                .next()
                .unwrap()
                .inner_html()
                .replace(" ", "_")
        ));

        println!("{document:#?}\n");
        let hello = document
            .select(&Selector::parse("a").unwrap())
            .nth(17)
            .unwrap();

        let rng = hello.inner_html().parse::<usize>().unwrap();

        // let rng = document
        //     .select(&Selector::parse("a").unwrap())
        //     .nth(17)
        //     .unwrap()
        //     .inner_html()
        //     .parse::<usize>()
        //     .unwrap();

        for i in 2..=rng {
            let body = self
                .client
                .get(format!("{}{i}", self.url))
                .send()
                .await?
                .text()
                .await?;
            // takes the response body and gets the fastest solutions
            Self::get_fastest_by_page(&body, &mut self.top_rust, &mut self.top_time)?;
            println!("parsed {i}th page successfully");

            // if let Err(e) = Self::get_fastest_by_page(&body, &mut self.top_rust, &mut self.top_time)
            // {
            //     println!("Error: {e}");
            //     if let Ok(_) = self.authenticate().await {
            //         Self::get_fastest_by_page(&body, &mut self.top_rust, &mut self.top_time)?;
            //     }
            // }
        }
        Ok(())
    }

    pub async fn create_json(&self) -> Result<(), Box<dyn Error>> {
        // this method is computed separately because in case something happens
        // and the user couldn't able to get the solutions, in that case the user
        // can get all the solutions without scraping all the solution webpages
        let dirname = self.dirname.as_ref().unwrap();
        tokio::fs::create_dir_all(format!("solutions/{dirname}/")).await?;
        let mut ftrs = File::create(&format!("solutions/{dirname}/top_rust.json")).await?;
        let mut fttm = File::create(&format!("solutions/{dirname}/top_time.json")).await?;

        // creating new vectors of SolnEntry to Serialize to a proper type
        #[derive(serde::Serialize)]
        struct SolnEntry {
            time: u8,
            chars: u16,
            url: String,
        }

        let top_rust_vec = self
            .top_rust
            .clone()
            .into_iter()
            .map(|((time, chars), url)| SolnEntry { time, chars, url })
            .collect::<Vec<SolnEntry>>();
        ftrs.write_all(
            serde_json::to_string_pretty(&top_rust_vec)
                .unwrap()
                .as_bytes(),
        )
        .await?;

        let top_time_vec = self
            .top_time
            .clone()
            .into_iter()
            .map(|((time, chars), url)| SolnEntry { time, chars, url })
            .collect::<Vec<SolnEntry>>();
        fttm.write_all(
            serde_json::to_string_pretty(&top_time_vec)
                .unwrap()
                .as_bytes(),
        )
        .await?;

        Ok(())
    }

    pub async fn write_all_files(self) -> Result<(), Box<dyn Error>> {
        // creates all the top rust files
        let client = Arc::new(self.client);
        let dirname = Arc::new(self.dirname.unwrap());
        println!("writing top rust files");
        for (i, ((time, len), purl)) in self.top_rust.into_iter().enumerate() {
            let client = Arc::clone(&client);
            let dirname = Arc::clone(&dirname);
            tokio::spawn(async move {
                let code = Self::get_code(client, purl).await.unwrap();
                let mut file = File::create(&format!(
                    "solutions/{dirname}/rust{i:03}-{time:02}-{len:05}.rs",
                ))
                .await
                .unwrap();
                file.write_all(code.as_bytes()).await.unwrap();
            });
        }
        // creates all the top time files
        println!("writing top time files");
        for (i, ((time, len), purl)) in self.top_time.into_iter().enumerate() {
            let client = Arc::clone(&client);
            let dirname = Arc::clone(&dirname);
            tokio::spawn(async move {
                let code = Self::get_code(client, purl).await.unwrap();
                let mut file = File::create(&format!(
                    "solutions/{dirname}/time{i:02}-{time:02}-{len:05}.cpp",
                ))
                .await
                .unwrap();
                file.write_all(code.as_bytes()).await.unwrap();
            });
        }
        Ok(())
    }

    pub async fn get_code(client: Arc<Client>, url: String) -> Result<String, Box<dyn Error>> {
        let body = client.as_ref().get(url).send().await?.text().await?;
        let fragment = Html::parse_document(&body);
        let code = fragment
            .select(&Selector::parse("pre").unwrap())
            .next()
            .unwrap()
            .inner_html()
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&amp;", "&");
        // this above two replace methods takes up a lot of time
        Ok(code)
    }
}
