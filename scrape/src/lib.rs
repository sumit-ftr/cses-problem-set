use reqwest;
use scraper::{Html, Selector};
use serde_json;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use tokio;

pub struct Config {
    url: String,
    num: usize,
    rng: usize,
    top_rust: BTreeMap<usize, String>,
    top_time: BTreeMap<usize, String>,
}

impl Config {
    pub async fn new(mut a: std::env::Args) -> Result<Self, Box<dyn Error>> {
        a.next();
        let (num, url) = match a.next() {
            Some(num) => {
                let num = num.parse::<usize>()?;
                (num, format!("https://cses.fi/problemset/hack/{num}/list/"))
            }
            None => return Err(Box::<dyn Error>::from("Error: Problem Number Not Found")),
        };
        let res = reqwest::get(&url).await?;
        let rng = Self::get_range(&res);
        Ok(Self {
            url,
            num,
            rng,
            top_rust: BTreeMap::new(),
            top_time: BTreeMap::new(),
        })
    }

    pub fn get_range(r: &reqwest::Response) -> usize {
        println!("{r:#?}");
        0
    }

    pub fn get_fastest(&mut self) {
        // key calculation: execution_time * 100 * no_of_chars
    }

    pub fn create_json(&self) -> Result<(), Box<dyn Error>> {
        // this method is computed separately because in case something happens
        // and the user couldn't able to get the solutions, in that case the user
        // can get all the solutions without scraping all the solution webpages
        fs::create_dir_all(format!("solutions/{}/", self.num))?;
        let mut file_top_rust = File::create(&format!("solutions/{}/top_rust.json", self.num))?;
        let mut file_top_time = File::create(&format!("solutions/{}/top_time.json", self.num))?;
        file_top_rust.write_all(serde_json::to_string(&self.top_rust).unwrap().as_bytes())?;
        file_top_time.write_all(serde_json::to_string(&self.top_time).unwrap().as_bytes())?;
        Ok(())
    }

    pub fn write_all_files(&self) {}
}
