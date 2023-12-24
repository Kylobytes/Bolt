/* api.rs
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

use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

use sha1::{Digest, Sha1};

use crate::config::{API_KEY, API_SECRET, BASE_URL};

pub fn build_url(endpoint: &str) -> String {
    BASE_URL.to_string() + endpoint
}

pub fn build_authentication_headers(
) -> Result<(String, String), SystemTimeError> {
    let date = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs()
        .to_string();

    let auth_string = format!("{}{}{}", API_KEY, API_SECRET, date);

    let mut hasher = Sha1::new();
    hasher.update(auth_string);

    let result = hasher.finalize();
    let authorization = format!("{:X}", result).to_lowercase();

    Ok((date, authorization))
}
