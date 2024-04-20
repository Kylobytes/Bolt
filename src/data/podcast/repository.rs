/* repository.rs
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

use crate::{
    api::{
        self, client,
        podcast::{response::PodcastResponse, Feed},
        search::{response::SearchResponse, result::SearchResult},
    },
    data::{database, podcast::provider},
};

pub async fn count() -> i32 {
    let Ok(pool) = database::connect().await else {
        return 0;
    };

    provider::count(&pool).await
}

pub async fn search(query: &str) -> Vec<SearchResult> {
    let endpoint: String = format!("/search/byterm?q={query}");
    let url: String = api::build_url(&endpoint);

    let Ok(response) = client().get(&url).send().await else {
        return vec![];
    };

    let Ok(results) = response.json::<SearchResponse>().await else {
        return vec![];
    };

    results.feeds
}

pub async fn subscribe(id: &i64) -> bool {
    let endpoint: String = format!("/podcasts/byfeedid?id={id}");
    let url: String = api::build_url(&endpoint);

    let Ok(connection) = database::connect().await else {
        return false;
    };

    let Ok(response) = client().get(&url).send().await else {
        return false;
    };

    let Ok(results) = response.json::<PodcastResponse>().await else {
        return false;
    };

    let feed: &Feed = &results.feed;
    let id: i64 = feed.id.clone();
    let name: String = feed.title.clone();
    let url: String = feed.url.clone();
    let description: Option<String> = if feed.description.is_empty() {
        None
    } else {
        Some(feed.description.clone())
    };

    let image: Option<String> = if feed.image.is_empty() {
        None
    } else {
        Some(feed.image.clone())
    };

    let Ok(_) = sqlx::query!(
        "INSERT INTO podcasts (\
             id,
             name,
             description,
             url,
             image_url\
         ) VALUES (?, ?, ?, ?, ?)",
        id,
        name,
        description,
        url,
        image
    )
    .execute(&connection)
    .await
    else {
        return false;
    };

    return true;
}

pub async fn unsubscribe(id: &i64) -> bool {
    let Ok(connection) = database::connect().await else {
        return false;
    };

    if let Err(_) =
        sqlx::query!("DELETE FROM episodes WHERE podcast_id = ?", id)
            .execute(&connection)
            .await
    {
        return false;
    };

    if let Err(_) = sqlx::query!("DELETE FROM podcasts WHERE id = ?", id)
        .execute(&connection)
        .await
    {
        return false;
    };

    true
}
