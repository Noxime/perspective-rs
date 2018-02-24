extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use reqwest::Client;
use reqwest::header::{ContentType};


/// All structs that represent the data returned by the API
pub mod model {
    use std::collections::HashMap;
    use std::fmt::{self, Debug, Display};

    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
    pub enum PerspectiveError {
        EmptyInput,
        EmptyTypes,
        RequestFailed,
        ParsingFailed(String),
    }

    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum ValueType {
        AttackOnAuthor,
        AttackOnCommenter,
        Incoherent,
        Inflammatory,
        LikelyToReject,
        Obscene,
        SevereToxicity,
        Spam,
        Toxicity,
        Unsubstantial,
    }

    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
    pub enum NumberType {
        #[serde(rename = "PROBABILITY")]
        Probability,
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

    impl From<ValueType> for String {
        fn from(v: ValueType) -> Self {
            match v {
                ValueType::AttackOnAuthor    => "ATTACK_ON_AUTHOR",
                ValueType::AttackOnCommenter => "ATTACK_ON_COMMENTER",
                ValueType::LikelyToReject    => "LIKELY_TO_REJECT",
                ValueType::Incoherent        => "INCOHERENT",
                ValueType::Inflammatory      => "INFLAMMATORY",
                ValueType::Obscene           => "OBSCENE",
                ValueType::SevereToxicity    => "SEVERE_TOXICITY",
                ValueType::Spam              => "SPAM",
                ValueType::Toxicity          => "TOXICITY",
                ValueType::Unsubstantial     => "UNSUBSTANTIAL",
                
            }.to_string()
        }
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
    do_not_store: bool
}

use model::*;

impl PerspectiveClient {
    
    /// create a new Perspective client with the given API key
    pub fn new(api_key: &str, do_not_store: bool) -> Self {
        Self {
            client: Client::new(),
            key: api_key.to_string(),
            do_not_store
        }
    }

    /// Send given text to Perspective API and return the requested values
    pub fn analyze(&self, text: &str, types: Vec<ValueType>) -> Result<Response, PerspectiveError> {
        if text.is_empty() {
            return Err(PerspectiveError::EmptyInput);
        }

        if types.len() == 0 {
            return Err(PerspectiveError::EmptyTypes);
        }

        const ENDPOINT: &'static str = "https://commentanalyzer.googleapis.com/v1alpha1/comments:analyze";

        let mut request = json!({
            "comment": { "text": text },
            "languages": [
                "en"
            ],
            "requestedAttributes": { 
                /*
                ValueType::AttackOnAuthor:    {},
                ValueType::AttackOnCommenter: {},
                ValueType::LikelyToReject:    {},
                ValueType::Incoherent:        {},
                ValueType::Inflammatory:      {},
                ValueType::Obscene:           {},
                ValueType::SevereToxicity:    {},
                ValueType::Spam:              {},
                ValueType::Toxicity:          {},
                ValueType::Unsubstantial:     {},
                */
            },
            "doNotStore": self.do_not_store
        });

        for type_ in types {
            request["requestedAttributes"].as_object_mut().unwrap().insert(String::from(type_), json!({}));
        }

        let mut ret = self.client.post(ENDPOINT)
            .query(&[("key", &self.key)])
            .header(ContentType::json())
            .body(request.to_string())
            .send().map_err(|_| PerspectiveError::RequestFailed)?;
        ret.json().map_err(|why| PerspectiveError::ParsingFailed(format!("{}", why)))
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::PerspectiveClient;
        use super::model::*;
        let api = PerspectiveClient::new(env!("PERSPECTIVE_KEY"), true);
        println!("{:#?}", api.analyze("thankfully rethinkdb and mongodb work very similar", 
            vec![ValueType::Toxicity, ValueType::Spam]));
    }
}
