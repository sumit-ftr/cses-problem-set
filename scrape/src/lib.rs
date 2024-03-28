use reqwest;
use scraper::{Html, Selector};
use std::collections::BTreeMap;
use std::error::Error;
use tokio;

pub struct Config {
    pub url: String,
    only_rust: bool,
    top_rust: BTreeMap<usize, String>,
    top_time: BTreeMap<usize, String>,
    range: usize,
}

impl Config {
    pub fn new(mut a: std::env::Args) -> Result<Self, Box<dyn Error>> {
        a.next();
        let url = match a.next() {
            Some(link) => link,
            None => return Err(Box::<dyn Error>::from("Error: Please provide an url")),
        };
        let only_rust = match a.next() {
            Some(v) => {
                if v == "-r" {
                    true
                } else {
                    false
                }
            }
            None => false,
        };
        Ok(Self {
            url,
            only_rust,
            range: Self::get_range(),
            top_rust: BTreeMap::new(),
            top_time: BTreeMap::new(),
        })
    }

    pub fn get_range() -> usize {
        todo!()
    }
}
