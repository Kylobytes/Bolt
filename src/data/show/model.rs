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

pub async fn load_shows(pool: &SqlitePool) -> Vec<Show> {
    let shows = sqlx::query_as!(
        Show,
        "SELECT \
         id, \
         name, \
         description, \
         url, \
         image_url \
         FROM shows",
    )
    .fetch_all(pool)
    .await
    .expect("Failed to load shows");

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
