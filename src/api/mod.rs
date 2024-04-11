pub mod episode;
pub mod episodes;
pub mod podcast;
pub mod podcasts;
pub mod search;

use std::sync::OnceLock;

use sha1::{
    digest::{
        core_api::CoreWrapper, generic_array::GenericArray, typenum::UInt,
    },
    Digest, Sha1, Sha1Core,
};
use time::OffsetDateTime;
use ureq::{Agent, AgentBuilder, Request};

use crate::config::{API_KEY, BASE_URL, USER_AGENT};

pub fn client() -> &'static Agent {
    static CLIENT: OnceLock<Agent> = OnceLock::new();

    CLIENT.get_or_init(|| AgentBuilder::new().build())
}

pub fn initiate_request(url: &str) -> Request {
    let date: i64 = OffsetDateTime::now_utc().unix_timestamp();
    let auth_string: String = format!("{}{}{}", &API_KEY, &API_KEY, &date);

    let mut hasher: CoreWrapper<Sha1Core> = Sha1::new();
    hasher.update(&auth_string);

    let result: GenericArray<u8, UInt<UInt<_, _>, _>> = hasher.finalize();
    let authorization: String = format!("{:X}", result).to_lowercase();

    client()
        .get(url)
        .set("User-Agent", &USER_AGENT)
        .set("X-Auth-Key", &API_KEY)
        .set("X-Auth-Date", &date.to_string())
        .set("Authorization", &authorization)
}

pub fn build_url(endpoint: &str) -> String {
    format!("{}{}", BASE_URL, endpoint)
}
