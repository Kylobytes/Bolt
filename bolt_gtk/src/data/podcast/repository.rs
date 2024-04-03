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

use bolt_entity::{
    episode,
    podcast::{ActiveModel, Column, Entity, Model},
};
use bolt_migration::Expr;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter,
};

use crate::{api::podcasts, data::database};

pub async fn subscribe(id: &i64) -> Result<Model, DbErr> {
    let response = podcasts::by_feed_id(id).await;
    let feed = response.feed;

    let podcast = ActiveModel {
        id: Set(feed.id),
        name: Set(feed.title),
        description: Set(Some(feed.description)
            .filter(|description| !description.is_empty())),
        url: Set(feed.url),
        image_url: Set(feed.image),
    };

    let connection = database::connect().await;
    podcast.insert(connection).await
}

pub async fn subscribed(id: &i64) -> bool {
    let connection = database::connect().await;

    if let Ok(count) = Entity::find_by_id(id.clone()).count(connection).await {
        return count > 0;
    };

    false
}

pub async fn load_count() -> u64 {
    let connection = database::connect().await;

    Entity::find()
        .count(connection)
        .await
        .expect("Failed to get podcast count")
}

pub async fn load_subscribed_ids(remote_ids: &Vec<i64>) -> Vec<i64> {
    let connection = database::connect().await;

    Entity::find()
        .filter(Expr::col(Column::Id).is_in(remote_ids.clone()))
        .all(connection)
        .await
        .expect("Failed to load podcast")
        .into_iter()
        .map(|show: Model| show.id)
        .collect()
}

pub async fn delete(id: &i64) {
    let connection = database::connect().await;

    episode::Entity::delete_many()
        .filter(Expr::col(episode::Column::PodcastId).eq(id.clone()))
        .exec(connection)
        .await
        .expect("Failed to delete related episodes");

    Entity::delete_by_id(id.clone())
        .exec(connection)
        .await
        .expect("Failed to delete podcast");
}
