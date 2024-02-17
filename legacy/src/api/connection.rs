/* connection.rs
 *
 * Copyright 2023 Kent Delante
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

use sha1::{Digest, Sha1};
use time::OffsetDateTime;

use crate::config::{API_KEY, API_SECRET, BASE_URL};

pub struct ApiConnection {
    pub url: String,
    pub auth_date: String,
    pub authorization: String,
}

impl ApiConnection {
    pub fn builder() -> ApiConnectionBuilder {
        ApiConnectionBuilder::default()
    }
}

#[derive(Default)]
pub struct ApiConnectionBuilder {
    url: String,
    auth_date: String,
    authorization: String,
}
impl ApiConnectionBuilder {
    pub fn build_url(mut self, endpoint: &str) -> Self {
        self.url = BASE_URL.to_string() + endpoint;

        self
    }

    pub fn build_authentication_headers(mut self) -> Self {
        let date = OffsetDateTime::now_utc().unix_timestamp();
        let auth_string = format!("{}{}{}", API_KEY, API_SECRET, date);

        let mut hasher = Sha1::new();
        hasher.update(auth_string);

        let result = hasher.finalize();
        let authorization = format!("{:X}", result).to_lowercase();

        self.auth_date = date.to_string();
        self.authorization = authorization;

        self
    }

    pub fn build(&self) -> ApiConnection {
        ApiConnection {
            url: self.url.to_owned(),
            auth_date: self.auth_date.to_owned(),
            authorization: self.authorization.to_owned(),
        }
    }
}
