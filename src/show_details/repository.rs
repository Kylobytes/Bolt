/* repository.rs
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

use crate::{
    api::{
        connection::ApiConnection,
        episode::{response::EpisodeResponse, Episode},
        AGENT,
    },
    config::{API_KEY, USER_AGENT},
    data::{database, show},
};

pub fn load_show_episodes(show_id: i64) -> Vec<Episode> {
    let endpoint = format!("/episodes/byfeedid?id={show_id}&max=100&pretty");

    let api_connection = ApiConnection::builder()
        .build_url(&endpoint)
        .build_authentication_headers()
        .build();

    let response: EpisodeResponse = AGENT
        .get(&api_connection.url)
        .set("User-Agent", USER_AGENT)
        .set("X-Auth-Key", API_KEY)
        .set("X-Auth-Date", &api_connection.auth_date)
        .set("Authorization", &api_connection.authorization)
        .call()
        .expect("Failed to download show's episodes")
        .into_json()
        .expect("Failed to parse show's episode response");

    response.items
}

pub fn check_subscribed(show_id: &i64) -> bool {
    let database = database::connect()
        .get()
        .expect("Failed to connect to database");

    show::model::check_subscribed(&database, show_id)
}
