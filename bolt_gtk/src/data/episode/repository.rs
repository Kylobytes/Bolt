/* repository.rs
 *
 * Copyright 2023 Kent Delante
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

use sea_orm::{EntityTrait, PaginatorTrait, QuerySelect};

use bolt_entity::{
    episode, podcast,
    prelude::{Episode, Podcast},
};

use crate::data::database;

pub async fn load_episode_count() -> u64 {
    let connection = database::connect().await;

    Episode::find()
        .count(connection)
        .await
        .expect("Failed to get podcast count")
}

pub async fn load_episodes(
    offset: &u64,
) -> Vec<(episode::Model, Option<podcast::Model>)> {
    let connection = database::connect().await;

    Episode::find()
        .find_also_related(Podcast)
        .limit(20)
        .offset(Some(offset.to_owned()))
        .all(connection)
        .await
        .expect("Failed to load episodes")
}
