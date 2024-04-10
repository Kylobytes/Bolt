/* mod.rs
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

pub mod response;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Episode {
    pub id: i64,
    pub title: String,
    pub link: String,
    pub description: String,
    pub guid: String,
    pub date_published: i64,
    pub date_published_pretty: String,
    pub date_crawled: i64,
    pub enclosure_url: String,
    pub enclosure_type: String,
    pub enclosure_length: i64,
    pub duration: Option<i64>,
    pub explicit: u8,
    pub episode: Option<i64>,
    pub episode_type: Option<String>,
    pub season: Option<i64>,
    pub image: String,
    pub feed_itunes_id: Option<i64>,
    pub feed_image: String,
    pub feed_id: i64,
    pub feed_language: String,
    pub feed_dead: i64,
    pub feed_duplicate_of: Option<i64>,
    pub chapters_url: Option<String>,
    pub transcription_url: Option<String>,
}
