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
 *https://api.podcastindex.org/api/1.0/recent/feeds?pretty
 * You should have received a copy of the GNU General Public License
 * along with Bolt. If not, see <https://www.gnu.org/licenses/>.
 *
 */

use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait, QuerySelect};

use bolt_entity::{
    episode,
    prelude::{Episode, Show},
    show,
};

pub async fn load_episode_count(connection: &DatabaseConnection) -> u64 {
    Episode::find()
        .count(connection)
        .await
        .expect("Failed to get show count")
}

pub async fn load_episodes(
    connection: &DatabaseConnection,
    offset: &u64,
) -> Vec<(episode::Model, Option<show::Model>)> {
    Episode::find()
        .find_also_related(Show)
        .limit(20)
        .offset(Some(offset.to_owned()))
        .all(connection)
        .await
        .expect("Failed to load episodes")
}
