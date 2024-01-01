/* episode_client.rs
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

use ureq::Agent;

use crate::{
    api::{
        connection::ApiConnection,
        episode::episodes_response::EpisodesResponse,
    },
    config::{API_KEY, USER_AGENT},
};

pub struct EpisodeClient;

impl EpisodeClient {
    pub fn fetch_recent(
        agent: &Agent,
        api_connection: &ApiConnection,
    ) -> EpisodesResponse {
        let response = agent
            .get(&api_connection.url)
            .set("User-Agent", USER_AGENT)
            .set("X-Auth-Date", &api_connection.auth_date)
            .set("X-Auth-Key", API_KEY)
            .set("Authorization", &api_connection.authorization)
            .call()
            .expect("Failed to get recent episodes from api")
            .into_string()
            .expect("Failed to parse api response");

        serde_json::from_str(&response)
            .expect("Failed to deserialize response content")
    }
}
