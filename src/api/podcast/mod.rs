/* mod.rs
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

pub mod response;

use std::collections::HashMap;

use serde::Deserialize;

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feed {
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
    pub last_http_status: i64,
    pub content_type: String,
    pub itunes_id: Option<i64>,
    pub generator: String,
    pub language: String,
    pub explicit: bool,
    pub medium: String,
    pub dead: i64,
    pub chash: String,
    pub episode_count: i64,
    pub crawl_errors: i64,
    pub parse_errors: i64,
    pub categories: HashMap<String, String>,
    pub locked: u8,
    pub image_url_hash: i64,
}
