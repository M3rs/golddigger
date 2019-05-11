use regex::RegexBuilder;
use serde::Deserialize;

pub mod db;
pub mod slack;

#[derive(Deserialize)]
pub struct Listing {
    pub data: Data,
}

impl Listing {
    pub fn get_codes(&self) -> Vec<String> {
        // Exclaimation points are valid?
        let pattern = r"(?P<code>([A-Z!]{4}-?){4})\s";
        let re = RegexBuilder::new(pattern)
            .case_insensitive(true)
            .build()
            .expect("Error creating regex!!");

        // TODO: get list of codes we've already done
        self.data
            .children
            .iter()
            .filter_map(|post| post.parse_code(&re))
            .collect()
    }
}

#[derive(Deserialize)]
pub struct Data {
    pub children: Vec<Post>,
}

#[derive(Deserialize)]
pub struct Post {
    pub data: PostData,
}

impl Post {
    pub fn parse_code(&self, re: &regex::Regex) -> Option<String> {
        let text = &self.data.selftext;
        if let Some(cap) = re.captures(&text) {
            if let Some(m) = cap.name("code") {
                return Some(m.as_str().to_string());
            }
        }

        return None;
    }
}

#[derive(Deserialize)]
pub struct PostData {
    pub selftext: String,
}
