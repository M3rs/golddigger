extern crate regex;
extern crate reqwest;
extern crate rusqlite;
extern crate serde_json;
extern crate toml;

mod golddigger;

use golddigger::db::Database;
use golddigger::slack;
use golddigger::Listing;

use std::io::prelude::*;
use std::fs::File;
use serde::Deserialize;

// TODO: move config to own file
#[derive(Debug, Deserialize)]
struct Config {
    slack_url: String,
    reddit_url: String,
    db_file: String,
}

impl Config {
    fn load(config_file: &str) -> Result<Config, Box<std::error::Error>> {
        let mut f = File::open(config_file)?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;
        let config: Config = toml::de::from_slice(&buffer)?;

        Ok(config)
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
    let config = Config::load("Config.toml")?;

    let body = reqwest::get(&config.reddit_url)?.text()?;
    let listing: Listing = serde_json::from_str(&body)?;

    let db = Database::new(&config.db_file)?;
    let existing = db.get_used()?;

    // TODO: only check posts once?
    let codes: Vec<_> = listing
        .get_codes()
        .iter()
        .filter(|c| !existing.contains(c.as_str()))
        .map(|c| c.to_string())
        .collect();

    if codes.len() > 0 {
        slack::notify(&codes, &config.slack_url)?;
        db.add_codes(&codes)?;
    }

    Ok(())
}
