/* response.rs
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

use std::collections::HashMap;

use serde::Deserialize;

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feed {
    pub id: i64,
    podcast_guid: String,
    pub title: String,
    pub url: String,
    original_url: String,
    link: String,
    pub description: String,
    author: String,
    owner_name: String,
    pub image: String,
    artwork: String,
    last_update_time: i64,
    last_crawl_time: i64,
    last_parse_time: i64,
    last_good_http_status_time: i64,
    last_http_status: i64,
    content_type: String,
    itunes_id: Option<i64>,
    generator: String,
    language: String,
    explicit: bool,
    medium: String,
    dead: i64,
    chash: String,
    episode_count: i64,
    crawl_errors: i64,
    parse_errors: i64,
    categories: HashMap<String, String>,
    locked: u8,
    image_url_hash: i64,
}

#[derive(Default, Debug, Deserialize)]
pub struct PodcastResponse {
    status: String,
    query: HashMap<String, String>,
    pub feed: Feed,
    description: String,
}
