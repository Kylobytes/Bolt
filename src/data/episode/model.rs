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
 *
 * You should have received a copy of the GNU General Public License
 * along with Bolt. If not, see <https://www.gnu.org/licenses/>.
 *
 */

use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;

use crate::{api::episode::Episode as ApiEpisode, data::episode::Episode};

pub fn save_episodes_for_show(
    database: &mut PooledConnection<SqliteConnectionManager>,
    episodes: &Vec<ApiEpisode>,
    show_id: &i64,
) {
    let transaction = database
        .transaction()
        .expect("Failed to start transaction to save episodes");

    for episode in episodes.iter() {
        let image: Option<String> = if episode.image.is_empty() {
            None
        } else {
            Some(episode.image.clone())
        };

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
                    image,
                    episode.date_published,
                    show_id
                ],
            )
            .expect("Failed to save episode");
    }

    let _ = transaction.commit();
}

pub fn load_episodes(
    database: &PooledConnection<SqliteConnectionManager>,
) -> Vec<Episode> {
    let mut statement = database
        .prepare(
            "SELECT \
         id, \
         title, \
         description, \
         url, \
         image_url, \
         date_published, \
         show_id \
         FROM episodes ORDER BY date_published DESC",
        )
        .expect("Failed to prepare select episodes statement");

    let rows = statement
        .query_map([], |row| {
            Ok(Episode {
                id: row.get::<usize, i64>(0)?,
                title: row.get::<usize, Option<String>>(1)?,
                description: row.get::<usize, Option<String>>(2)?,
                url: row.get::<usize, Option<String>>(3)?,
                image_url: row.get::<usize, Option<String>>(4)?,
                date_published: row.get::<usize, i64>(5)?,
                show_id: row.get::<usize, i64>(6)?,
            })
        })
        .expect("Failed to load episodes");

    let mut episodes: Vec<Episode> = vec![];

    for row in rows {
        if let Ok(episode) = row {
            episodes.push(episode);
        }
    }

    episodes
}
