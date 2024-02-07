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
        episode::{response::EpisodeResponse, Episode},
        podcast::response::PodcastResponse,
        search::{response::SearchResponse, result::SearchResult},
        CLIENT,
    },
    config::{API_KEY, USER_AGENT},
    data::{
        database, episode,
        show::{self, Show},
    },
    utils::{self, episode_image_path},
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
    let pool = database::connect().await;

    let show_ids: Vec<i64> = show::model::load_shows(&pool)
        .await
        .into_iter()
        .map(|show: Show| show.id)
        .collect();

    show_ids
}

pub async fn subscribe(show_id: &i64) {
    let endpoint = format!("/podcasts/byfeedid?id={show_id}");

    let api_connection = ApiConnection::builder()
        .build_url(&endpoint)
        .build_authentication_headers()
        .build();

    let response: PodcastResponse = CLIENT
        .get(&api_connection.url)
        .header(header::USER_AGENT, USER_AGENT)
        .header(AUTHORIZATION, &api_connection.authorization)
        .header("X-Auth-Key", API_KEY)
        .header("X-Auth-Date", &api_connection.auth_date)
        .send()
        .await
        .expect("Failed to request podcast")
        .json()
        .await
        .expect("Failed to parse podcast");

    let pool = database::connect().await;

    show::model::save_subscription(&pool, &response).await;

    if !response.feed.image.is_empty() {
        let image_path = utils::show_image_path(&show_id.to_string());
        let _ = utils::save_image(&response.feed.image, &image_path);
    }

    let episodes_endpoint =
        format!("/episodes/byfeedid?id={show_id}&max=1000&fulltext");

    let episodes_connection = ApiConnection::builder()
        .build_url(&episodes_endpoint)
        .build_authentication_headers()
        .build();

    let episode_response: EpisodeResponse = CLIENT
        .get(&episodes_connection.url)
        .header(header::USER_AGENT, USER_AGENT)
        .header(AUTHORIZATION, &api_connection.authorization)
        .header("X-Auth-Key", API_KEY)
        .header("X-Auth-Date", &api_connection.auth_date)
        .send()
        .await
        .expect("Failed to load episodes")
        .json()
        .await
        .expect("Failed to parse episodes");

    episode::model::save_episodes_for_show(
        &pool,
        &episode_response.items,
        &show_id,
    )
    .await;

    let episodes_with_image: Vec<Episode> = episode_response
        .items
        .into_iter()
        .filter(|episode| !episode.image.is_empty())
        .collect();

    for episode in episodes_with_image.into_iter() {
        let path = episode_image_path(&episode.id.to_string());

        let _ = utils::save_image(&episode.image, &path).await;
    }
}
