pub mod episode;
pub mod episodes;
pub mod podcast;
pub mod podcasts;
pub mod search;

use std::sync::OnceLock;

use reqwest::{Client, RequestBuilder};
use sha1::{
    digest::{
        core_api::CoreWrapper, generic_array::GenericArray, typenum::UInt,
    },
    Digest, Sha1, Sha1Core,
};
use time::OffsetDateTime;

use crate::config::{API_KEY, BASE_URL, USER_AGENT};

pub fn client() -> &'static Client {
    static CLIENT: OnceLock<Client> = OnceLock::new();

    CLIENT.get_or_init(|| Client::new())
}

pub fn initiate_request(url: &str) -> RequestBuilder {
    let date: i64 = OffsetDateTime::now_utc().unix_timestamp();
    let auth_string: String = format!("{}{}{}", &API_KEY, &API_KEY, &date);

    let mut hasher: CoreWrapper<Sha1Core> = Sha1::new();
    hasher.update(&auth_string);

    let result: GenericArray<u8, UInt<UInt<_, _>, _>> = hasher.finalize();
    let authorization: String = format!("{:X}", result).to_lowercase();

    client()
        .get(url)
        .header("User-Agent", USER_AGENT)
        .header("X-Auth-Key", API_KEY)
        .header("X-Auth-Date", &date.to_string())
        .header("Authorization", &authorization)
}

pub fn build_url(endpoint: &str) -> String {
    format!("{}{}", BASE_URL, endpoint)
}
