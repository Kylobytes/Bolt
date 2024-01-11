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
 */

use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;

use crate::api::podcast::response::PodcastResponse;

pub fn save_subscription(
    database: &PooledConnection<SqliteConnectionManager>,
    podcast: &PodcastResponse,
) {
    let feed = &podcast.feed;

    let mut statement = database
        .prepare(
            "INSERT INTO shows (\
             id, \
             name, \
             description, \
             url,  \
             image_url \
             ) VALUES (?,?,?,?,?)",
        )
        .expect("Failed to prepare save subscription statement");

    statement
        .execute(params![
            feed.id,
            feed.title,
            feed.description,
            feed.url,
            feed.image
        ])
        .expect("Failed to save subscription");
}
