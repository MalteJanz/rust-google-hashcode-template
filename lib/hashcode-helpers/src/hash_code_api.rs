use reqwest::header::{self, HeaderMap, HeaderValue};
use reqwest::Response;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::time::Duration;
use std::collections::HashMap;

// inspired by https://github.com/svissers/hash-code-2018/blob/master/submit.py

pub struct HashCodeApiClient {
    client: Client
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RoundsResponse {
    Items(Vec<Round>),
    Error {
        code: u32,
        message: String,
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Round {
    id: String,
    name: String,
    data_sets: Vec<DataSet>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSet {
    id: String,
    name: String,
}

impl HashCodeApiClient {
    // TODO: implement error handling instead of crashing.
    pub fn new(token: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).expect("invalid token to use in header"));
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(10))
            .default_headers(headers).build().unwrap();

        let res = client.get("https://hashcode-judge.appspot.com/api/judge/v1/rounds").send().unwrap();
        //println!("{}", res.text().unwrap());

        let res_obj: RoundsResponse = res.json().unwrap();
        println!("{:#?}", res_obj);

        Self {
            client
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let client = HashCodeApiClient::new("TOKEN");
    }
}

