/* podcasts.rs
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

use super::{
    build_url, initiate_request, podcast::response::PodcastResponse,
    search::response::SearchResponse,
};

pub async fn search(query: &str) -> SearchResponse {
    let endpoint = format!("/search/byterm?q={query}");
    let url = build_url(&endpoint);
    let client = initiate_request(&url);

    client
        .send()
        .await
        .unwrap()
        .json::<SearchResponse>()
        .await
        .unwrap()
}

pub async fn by_feed_id(id: &i64) -> PodcastResponse {
    let endpoint = format!("/podcasts/byfeedid?id={id}");
    let url = build_url(&endpoint);
    let client = initiate_request(&url);

    client
        .send()
        .await
        .unwrap()
        .json::<PodcastResponse>()
        .await
        .unwrap()
}
