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

use reqwest::header::{self, AUTHORIZATION};

use crate::{
    api::{
        connection::ApiConnection,
        episode::{response::EpisodeResponse, Episode},
        CLIENT,
    },
    config::{API_KEY, USER_AGENT},
    data::{database, show},
};

pub async fn load_show_episodes(show_id: i64) -> Vec<Episode> {
    let endpoint = format!("/episodes/byfeedid?id={show_id}&max=100&pretty");

    let api_connection = ApiConnection::builder()
        .build_url(&endpoint)
        .build_authentication_headers()
        .build();

    let response: EpisodeResponse = CLIENT
        .get(&api_connection.url)
        .header(header::USER_AGENT, USER_AGENT)
        .header(AUTHORIZATION, &api_connection.authorization)
        .header("X-Auth-Key", API_KEY)
        .header("X-Auth-Date", &api_connection.auth_date)
        .send()
        .await
        .expect("Failed to download episodes")
        .json()
        .await
        .expect("Failed to parse episodes");

    response.items
}

pub async fn check_subscribed(show_id: &i64) -> bool {
    let pool = database::connect().await;

    show::model::subscribed(&pool, show_id).await
}
