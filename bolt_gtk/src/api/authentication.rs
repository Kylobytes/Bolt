/* authentication.rs
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

use sha1::{Digest, Sha1};
use time::OffsetDateTime;

use crate::config::{API_KEY, API_SECRET, USER_AGENT};

#[derive(Debug)]
pub struct RequestHeaders {
    pub user_agent: String,
    pub auth_key: String,
    pub auth_date: String,
    pub authorization: String,
}

impl RequestHeaders {
    pub fn new() -> Self {
        let date = OffsetDateTime::now_utc().unix_timestamp();
        let auth_string = format!("{}{}{}", API_KEY, API_SECRET, date);

        let mut hasher = Sha1::new();
        hasher.update(auth_string);

        let result = hasher.finalize();
        let authorization = format!("{:X}", result).to_lowercase();

        Self {
            user_agent: USER_AGENT.to_string(),
            auth_key: API_KEY.to_string(),
            auth_date: date.to_string(),
            authorization,
        }
    }
}
