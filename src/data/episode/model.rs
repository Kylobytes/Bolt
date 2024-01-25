/* model.rs
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
 *https://api.podcastindex.org/api/1.0/recent/feeds?pretty
 * You should have received a copy of the GNU General Public License
 * along with Bolt. If not, see <https://www.gnu.org/licenses/>.
 *
 */

use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;

use crate::api::episode::Episode;

pub fn save_episodes_for_show(
    database: &mut PooledConnection<SqliteConnectionManager>,
    episodes: &Vec<Episode>,
    show_id: &i64,
) {
    let transaction = database
        .transaction()
        .expect("Failed to start transaction to save episodes");

    for episode in episodes.iter() {
        transaction
            .execute(
                "INSERT INTO episodes (\
             id, \
             title, \
             description, \
             url, \
             image_url, \
             date_published, \
             show_id\
             ) VALUES (?,?,?,?,?,?,?)",
                params![
                    episode.id,
                    episode.title,
                    episode.description,
                    episode.link,
                    episode.image,
                    episode.date_published,
                    show_id
                ],
            )
            .expect("Failed to save episode");
    }

    let _ = transaction.commit();
}
