/* repository.rs
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

use std::path::PathBuf;

use crate::{
    api::{
        connection::ApiConnection,
        search::{result::SearchResult, results::SearchResults},
        AGENT,
    },
    config::{API_KEY, USER_AGENT},
};

pub struct DiscoverRepository;

impl DiscoverRepository {
    pub fn search_shows(query: &str) -> Vec<SearchResult> {
        let endpoint = format!(
            "/search/byterm?q={}",
            &query.to_string().replace(" ", "+")
        );

        let api_connection = ApiConnection::builder()
            .build_url(&endpoint)
            .build_authentication_headers()
            .build();

        let response: SearchResults = AGENT
            .get(&api_connection.url)
            .set("User-Agent", USER_AGENT)
            .set("X-Auth-Key", API_KEY)
            .set("X-Auth-Date", &api_connection.auth_date)
            .set("Authorization", &api_connection.authorization)
            .call()
            .expect("Failed to download shows from api")
            .into_json()
            .expect("Failed to parse response");

        response.feeds
    }

    pub fn save_image(url: &str, path: &PathBuf) -> Result<(), ureq::Error> {
        let mut response = AGENT.get(url).call()?.into_reader();

        let mut image = std::fs::File::create(&path)
            .expect("Failed to create image at path");

        std::io::copy(&mut response, &mut std::io::BufWriter::new(&mut image))
            .expect("Failed to save image");

        Ok(())
    }
}
