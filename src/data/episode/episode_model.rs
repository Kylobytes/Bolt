/* episode_model.rs
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

use crate::{api::episode::Episode, data::show::show_model::ShowModel};

#[derive(Clone, Debug)]
pub struct EpisodeModel {
    pub id: i64, // podcastindex.org's id
    pub title: String,
    pub description: Option<String>,
    pub url: String,
    pub guid: Option<String>,
    pub image_url: Option<String>,
    pub date_published: i64,
    pub show: Option<ShowModel>,
}

impl From<Episode> for EpisodeModel {
    fn from(episode: Episode) -> Self {
        EpisodeModel {
            id: episode.id,
            title: episode.title,
            description: Some(episode.description)
                .filter(|description| !description.is_empty()),
            url: episode.link,
            guid: Some(episode.guid).filter(|guid| !guid.is_empty()),
            image_url: Some(episode.image).filter(|image| !image.is_empty()),
            date_published: episode.date_published,
            show: Some(ShowModel {
                id: episode.feed_id,
                name: episode.feed_title,
                description: None,
                url: None,
                image_url: Some(episode.feed_image)
                    .filter(|image| !image.is_empty()),
            }),
        }
    }
}
