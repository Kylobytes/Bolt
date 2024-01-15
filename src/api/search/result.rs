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
    pub podcast_guid: String,
    pub title: String,
    pub url: String,
    pub original_url: String,
    pub link: String,
    pub description: String,
    pub author: String,
    pub owner_name: String,
    pub image: String,
    pub artwork: String,
    pub last_update_time: i64,
    pub last_crawl_time: i64,
    pub last_parse_time: i64,
    pub last_good_http_status_time: i64,
    pub content_type: String,
    pub itunes_id: Option<u64>,
    pub language: String,
    pub explicit: bool,
    #[serde(alias = "type")]
    pub show_type: u8,
    pub medium: String,
    pub dead: u8,
    pub episode_count: u64,
    pub crawl_errors: u64,
    pub parse_errors: u64,
    pub categories: Option<HashMap<String, String>>,
    pub image_url_hash: u64,
    #[serde(alias = "newestItemPubdate")]
    pub newest_item_pub_date: i64,
}
