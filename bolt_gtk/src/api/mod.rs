pub mod authentication;
pub mod episode;
pub mod podcast;
pub mod podcasts;
pub mod search;

use once_cell::sync::Lazy;
use reqwest::{Client, RequestBuilder};

use crate::config::BASE_URL;

use self::authentication::RequestHeaders;

pub static CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

pub fn initiate_request(url: &str) -> RequestBuilder {
    let headers = RequestHeaders::new();

    CLIENT
        .get(url)
        .header("User-Agent", headers.user_agent)
        .header("X-Auth-Key", headers.auth_key)
        .header("X-Auth-Date", headers.auth_date)
        .header("Authorization", headers.authorization)
}

pub fn build_url(endpoint: &str) -> String {
    format!("{}{}", BASE_URL, endpoint)
}
