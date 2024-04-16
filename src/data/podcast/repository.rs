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
