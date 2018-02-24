extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use reqwest::Client;
use reqwest::header::{ContentType};

/// All structs that represent the data returned by the API
pub mod model {
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
    pub enum PerspectiveError {
        EmptyInput,
        RequestFailed,
        ParsingFailed,
    }

    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
    pub enum ValueType {
        TOXICITY,
    }

    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
    pub enum NumberType {
        PROBABILITY,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub struct Score {
        pub value: f64,
        #[serde(rename = "type")]
        pub type_: NumberType,
    }
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub struct Scores {
        #[serde(rename = "summaryScore")]
        pub summary: Score,
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Response {
        #[serde(rename = "attributeScores")]
        pub scores: HashMap<ValueType, Scores>
    }
}


/// Perspective-rs
/// 
/// A very basic library for accessing [Google's Perspective API](https://www.perspectiveapi.com/)
/// This crate is built only for my needs, so many features might be missing or incomplete. If there is a feature or improvement you want, you can either
/// 
/// * Open an issue
/// * Open a pull request
/// * Message me on Discord (`noxim#6410`)
/// 
pub struct PerspectiveClient {
    client: Client,
    key: String,
}

use model::*;

impl PerspectiveClient {
    
    /// create a new Perspective client with the given API key
    pub fn new(api_key: &str) -> Self {
        Self {
            client: Client::new(),
            key: api_key.to_string(),
        }
    }

    /// Send given text to Perspective API and return the requested values
    pub fn analyze(&self, text: &str) -> Result<Response, PerspectiveError> {
        if text.is_empty() {
            return Err(PerspectiveError::EmptyInput);
        }
        const ENDPOINT: &'static str = "https://commentanalyzer.googleapis.com/v1alpha1/comments:analyze";

        let mut ret = self.client.post(ENDPOINT)
            .query(&[("key", &self.key)])
            .header(ContentType::json())
            .body(format!(r#"{{comment:{{text:"{}"}},languages:["en"],requestedAttributes:{{TOXICITY:{{}}}}}}"#, text))
            .send().map_err(|_| PerspectiveError::RequestFailed)?;
        ret.json().map_err(|_| PerspectiveError::ParsingFailed)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::PerspectiveClient;
        let api = PerspectiveClient::new(env!("PERSPECTIVE_KEY"));
        println!("{:#?}", api.analyze("thankfully rethinkdb and mongodb work very similar"));
    }
}
