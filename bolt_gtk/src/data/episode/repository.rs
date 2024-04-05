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

use rss::Channel;
use sea_orm::{EntityTrait, PaginatorTrait, QuerySelect, Set};
use time::{
    macros::format_description,
    OffsetDateTime,
};

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

pub async fn save_from_channel(feed: &Channel, podcast_id: &i64) {
    let episodes: Vec<episode::ActiveModel> = feed
        .items
        .clone()
        .into_iter()
        .map(|item| {
            let enclosure = item.enclosure.expect("Failed to get enclosure");
            let date_format = format_description!("[weekday repr:short], [day] [month repr:short] [year] [hour]:[minute]:[second] [offset_hour][offset_minute]");
            let publish_date: i64 = match OffsetDateTime::parse(
                &item.pub_date.expect("Failed to get publish date"),
                &date_format
            ) {
                Ok(datetime) => {
                    let timestamp_format =
                        format_description!("[unix_timestamp]");
                    datetime
                        .format(timestamp_format)
                        .unwrap()
                        .parse::<i64>()
                        .expect("Failed to parse date")
                },
                Err(_) => 0
            };

            episode::ActiveModel {
                id: Default::default(),
                title: Set(item.title.expect("Failed to get title")),
                description: Set(item.description),
                url: Set(item.link.expect("Failed to get link")),
                image_url: Set(None),
                enclosure_url: Set(enclosure.url),
                queued: Set(false),
                date_published: Set(publish_date),
                podcast_id: Set(podcast_id.to_owned()),
            }
        })
        .collect();

    let connection = database::connect().await;

    Episode::insert_many(episodes)
        .exec(connection)
        .await
        .expect("Failed to save episodes");
}
