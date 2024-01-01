/* episode_data.rs
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

use rusqlite::{params, Transaction};

use crate::data::episode::episode_model::EpisodeModel;

pub struct EpisodeData;

impl EpisodeData {
    pub fn save_transaction(
        transaction: &Transaction,
        episode: &EpisodeModel,
    ) {
        let mut statement = transaction
            .prepare(
                "REPLACE INTO episodes (\
                 id, \
                 title, \
                 description, \
                 url, \
                 guid, \
                 image_url, \
                 date_published, \
                 show_id\
                 ) VALUES (?,?,?,?,?,?,?,?)",
            )
            .expect("Failed to prepare query");

        let id = if let Some(show) = &episode.show {
            Some(show.id)
        } else {
            None
        };

        statement
            .execute(params![
                episode.id,
                episode.title,
                episode.description,
                episode.url,
                episode.guid,
                episode.image_url,
                episode.date_published,
                id
            ])
            .expect("Failed to save episode");
    }
}
