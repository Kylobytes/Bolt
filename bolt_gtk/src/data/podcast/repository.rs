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

use bolt_entity::podcast::{Entity, Model};
use sea_orm::{EntityTrait, PaginatorTrait};

use crate::data::database;

pub async fn load_show_count() -> u64 {
    let connection = database::connect().await;

    Entity::find()
        .count(connection)
        .await
        .expect("Failed to get show count")
}

pub async fn load_subscribed_ids() -> Vec<i64> {
    let connection = database::connect().await;

    Entity::find()
        .all(connection)
        .await
        .expect("Failed to load shows")
        .into_iter()
        .map(|show: Model| show.id)
        .collect()
}
