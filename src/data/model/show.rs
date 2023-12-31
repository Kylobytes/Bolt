/* show.rs
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

use std::error::Error;

use rusqlite::{params, Transaction};

use crate::api::episode::Episode as RemoteEpisode;

#[derive(Debug, Clone)]
pub struct Show {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub url: Option<String>,
    pub image_url: Option<String>,
}

impl From<RemoteEpisode> for Show {
    fn from(episode: RemoteEpisode) -> Self {
        Show {
            id: episode.feed_id,
            name: episode.feed_title,
            description: None,
            url: None,
            image_url: Some(episode.feed_image)
                .filter(|image| !image.is_empty()),
        }
    }
}

impl Show {
    pub fn save_transaction(
        &self,
        transaction: &Transaction,
    ) -> Result<usize, Box<dyn Error>> {
        let mut statement = transaction.prepare(
            "REPLACE INTO shows (\
             id, \
             name, \
             description, \
             url, \
             image_url \
             ) VALUES (?,?,?,?,?)",
        )?;

        Ok(statement.execute(params![
            self.id,
            self.name,
            self.description,
            self.url,
            self.image_url
        ])?)
    }
}
