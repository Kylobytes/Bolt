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

use reqwest::header::{self, AUTHORIZATION};

use crate::{
    api::{
        connection::ApiConnection,
        episode::response::EpisodeResponse,
        podcast::response::PodcastResponse,
        search::{response::SearchResponse, result::SearchResult},
        AGENT, CLIENT,
    },
    config::{API_KEY, USER_AGENT},
    data::{
        database, episode,
        show::{self, Show},
    },
    utils,
};

pub async fn search_shows(query: &str) -> Vec<SearchResult> {
    let endpoint =
        format!("/search/byterm?q={}", &query.to_string().replace(" ", "+"));

    let api_connection = ApiConnection::builder()
        .build_url(&endpoint)
        .build_authentication_headers()
        .build();

    let response: SearchResponse = CLIENT
        .get(&api_connection.url)
        .header(header::USER_AGENT, USER_AGENT)
        .header(AUTHORIZATION, &api_connection.authorization)
        .header("X-Auth-Key", API_KEY)
        .header("X-Auth-Date", &api_connection.auth_date)
        .send()
        .await
        .expect("Failed to search podcasts")
        .json()
        .await
        .expect("Failed to parse search results");

    response.feeds
}

pub async fn load_subscribed_show_ids() -> Vec<i64> {
    let database = database::connect_async().await;

    let show_ids: Vec<i64> = show::model::load_shows(&database)
        .await
        .into_iter()
        .map(|show: Show| show.id)
        .collect();

    show_ids
}

pub fn subscribe(show_id: &i64) {
    let endpoint = format!("/podcasts/byfeedid?id={show_id}");

    let api_connection = ApiConnection::builder()
        .build_url(&endpoint)
        .build_authentication_headers()
        .build();

    let response: PodcastResponse = AGENT
        .get(&api_connection.url)
        .set("User-Agent", USER_AGENT)
        .set("X-Auth-Key", API_KEY)
        .set("X-Auth-Date", &api_connection.auth_date)
        .set("Authorization", &api_connection.authorization)
        .call()
        .expect("Failed to subscribe to podcast")
        .into_json()
        .expect("Failed to parse search results");

    let mut database = database::connect()
        .get()
        .expect("Failed to connect to database");

    show::model::save_subscription(&database, &response);

    if !response.feed.image.is_empty() {
        let image_path = utils::show_image_path(&show_id.to_string());
        let _ = utils::save_image(&response.feed.image, &image_path);
    }

    let episodes_endpoint = format!("/episodes/byfeedid?id={show_id}");

    let episodes_connection = ApiConnection::builder()
        .build_url(&episodes_endpoint)
        .build_authentication_headers()
        .build();

    let episode_response: EpisodeResponse = AGENT
        .get(&episodes_connection.url)
        .set("User-Agent", USER_AGENT)
        .set("X-Auth-Key", API_KEY)
        .set("X-Auth-Date", &api_connection.auth_date)
        .set("Authorization", &api_connection.authorization)
        .call()
        .expect("Failed to subscribe to podcast")
        .into_json()
        .expect("Failed to parse search results");

    episode::model::save_episodes_for_show(
        &mut database,
        &episode_response.items,
        &show_id,
    );
}
