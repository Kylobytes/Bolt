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

use sqlx::SqlitePool;

use crate::{api::podcast::response::PodcastResponse, data::show::Show};

pub async fn save_subscription(pool: &SqlitePool, podcast: &PodcastResponse) {
    let feed = &podcast.feed;

    sqlx::query!(
        "INSERT INTO shows (\
             id, \
             name, \
             description, \
             url, \
             image_url \
             ) VALUES (?,?,?,?,?)",
        feed.id,
        feed.title,
        feed.description,
        feed.url,
        feed.image
    )
    .execute(pool)
    .await
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

pub async fn subscribed(pool: &SqlitePool, id: &i64) -> bool {
    let show =
        sqlx::query!("SELECT COUNT(id) AS count FROM shows WHERE id = ?", id)
            .fetch_one(pool)
            .await
            .expect("Failed to check if subscribed to show");

    show.count == 1
}

pub async fn load_show_count(pool: &SqlitePool) -> i32 {
    let result = sqlx::query!("SELECT COUNT(id) AS count FROM shows")
        .fetch_one(pool)
        .await
        .expect("Failed to query show count");

    result.count
}
