/* result.rs
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

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub id: i64,
    podcast_guid: String,
    pub title: String,
    url: String,
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
    content_type: String,
    itunes_id: Option<u64>,
    language: String,
    explicit: bool,
    #[serde(alias = "type")]
    show_type: u8,
    medium: String,
    dead: u8,
    episode_count: u64,
    crawl_errors: u64,
    parse_errors: u64,
    categories: Option<HashMap<String, String>>,
    image_url_hash: u64,
    #[serde(alias = "newestItemPubdate")]
    newest_item_pub_date: i64,
}
