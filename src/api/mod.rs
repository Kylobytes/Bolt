/* mod.rs
 *
 * Copyright 2024 Kent Delante
 *
 * This file is part of Bolt.
 *
 * Bolt is free software: you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Bolt is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Bolt. If not, see <https://www.gnu.org/licenses/>.
 */

pub mod episode;
pub mod podcast;
pub mod search;

use std::sync::OnceLock;

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use sha1::{
    digest::{
        core_api::CoreWrapper, generic_array::GenericArray, typenum::UInt,
    },
    Digest, Sha1, Sha1Core,
};
use time::OffsetDateTime;

use crate::config::{API_KEY, API_SECRET, BASE_URL, USER_AGENT};

pub fn client() -> &'static Client {
    static CLIENT: OnceLock<Client> = OnceLock::new();

    CLIENT.get_or_init(|| {
        let time: i64 = OffsetDateTime::now_utc().unix_timestamp();
        let auth_string: String =
            format!("{}{}{}", &API_KEY, &API_SECRET, &time);

        let mut hasher: CoreWrapper<Sha1Core> = Sha1::new();
        hasher.update(&auth_string);

        let hash: GenericArray<u8, UInt<UInt<_, _>, _>> = hasher.finalize();
        let authorization: String = format!("{:X}", hash).to_lowercase();

        let mut headers = HeaderMap::new();
        headers.insert("User-Agent", HeaderValue::from_static(USER_AGENT));
        headers.insert("X-Auth-Key", HeaderValue::from_static(API_KEY));

        if let Ok(date) = HeaderValue::from_str(&time.to_string()) {
            headers.insert("X-Auth-Date", date);
        }

        if let Ok(authorization) = HeaderValue::from_str(&authorization) {
            headers.insert("Authorization", authorization);
        }

        Client::builder()
            .default_headers(headers)
            .build()
            .expect("Client must be created to perform requests")
    })
}

pub fn build_url(endpoint: &str) -> String {
    format!("{}{}", BASE_URL, endpoint)
}
