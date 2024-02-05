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
use sqlx::SqlitePool;

use crate::{api::podcast::response::PodcastResponse, data::show::Show};

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
             url, \
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
            feed.image,
        ])
        .expect("Failed to save subscription");
}

pub fn load_shows(
    database: &PooledConnection<SqliteConnectionManager>,
) -> Vec<Show> {
    let mut statement = database
        .prepare(
            "SELECT \
             id, \
             name, \
             description, \
             url, \
             image_url \
             FROM shows",
        )
        .expect("Failed to prepare select statement (shows)");

    let rows = statement
        .query_map([], |row| {
            Ok(Show {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                url: row.get(3)?,
                image_url: row.get(4)?,
            })
        })
        .expect("Failed to load show ids");

    let mut shows: Vec<Show> = vec![];

    for row in rows {
        if let Ok(show) = row {
            shows.push(show);
        }
    }

    shows
}

pub fn check_subscribed(
    database: &PooledConnection<SqliteConnectionManager>,
    id: &i64,
) -> bool {
    let mut statement = database
        .prepare("SELECT COUNT(id) FROM shows WHERE id = ?")
        .unwrap();
    let count = statement
        .query_row(params![id], |row| {
            Ok(row.get::<usize, i64>(0).expect("Failed to get count"))
        })
        .expect("Failed to check if show id exists");

    count == 1
}

pub async fn load_show_count(pool: &SqlitePool) -> i32 {
    let result = sqlx::query!("SELECT COUNT(id) AS count FROM shows")
        .fetch_one(pool)
        .await
        .expect("Failed to query show count");

    result.count
}
