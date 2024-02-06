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

use sqlx::{query, Executor, SqlitePool};

use crate::{api::episode::Episode as ApiEpisode, data::episode::Episode};

pub async fn save_episodes_for_show(
    pool: &SqlitePool,
    episodes: &Vec<ApiEpisode>,
    show_id: &i64,
) {
    let mut transaction =
        pool.begin().await.expect("Failed to begin transaction");

    for episode in episodes.iter() {
        transaction
            .execute(query!(
                "INSERT INTO episodes (\
             id, \
             title, \
             description, \
             url, \
             image_url, \
             date_published, \
             show_id\
             ) VALUES (?,?,?,?,?,?,?)",
                episode.id,
                episode.title,
                episode.description,
                episode.link,
                episode.image,
                episode.date_published,
                show_id
            ))
            .await
            .expect("Failed to save episode");
    }

    let _ = transaction.commit().await.expect("Failed to save episodes");
}

pub async fn load_episodes(pool: &SqlitePool) -> Vec<Episode> {
    let episodes = sqlx::query_as!(
        Episode,
        "SELECT \
         id, \
         title, \
         description, \
         url, \
         image_url, \
         date_published, \
         show_id \
         FROM episodes ORDER BY date_published DESC LIMIT 100"
    )
    .fetch_all(pool)
    .await
    .expect("Failed to load episodes");

    episodes
}
