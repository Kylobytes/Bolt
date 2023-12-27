/* show_repository.rs
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

use std::error::Error;

use rusqlite::params;
use ureq::AgentBuilder;

use crate::{
    config::{API_KEY, USER_AGENT},
    data::{
        database,
        model::{episode::Episode, show::Show},
        remote::{
            api::{build_authentication_headers, build_url},
            recent_episodes::RecentEpisodes,
        },
    },
};

pub fn fetch_latest_episodes() -> Result<Vec<Episode>, Box<dyn Error>> {
    let agent = AgentBuilder::new().build();
    let (date, authorization) = build_authentication_headers()?;
    let url = build_url("/recent/episodes?max=10");

    let response = agent
        .get(url.as_str())
        .set("User-Agent", USER_AGENT)
        .set("X-Auth-Date", date.as_str())
        .set("X-Auth-Key", API_KEY)
        .set("Authorization", authorization.as_str())
        .call()?
        .into_string()?;

    let recent_episodes: RecentEpisodes = serde_json::from_str(&response)?;
    let pool = database::connect();
    let mut database = pool.get()?;
    let transaction = database.transaction()?;

    for episode in recent_episodes.items.iter() {
        episode.save_episode_transaction(&transaction)?;
        episode.save_show_transaction(&transaction)?;
        episode.save_image()?;
    }

    transaction.commit()?;

    let mut statement = database.prepare(
        "SELECT \
         episodes.id, \
         episodes.title, \
         episodes.description, \
         episodes.url, \
         episodes.guid, \
         episodes.image_url, \
         episodes.date_published, \
         shows.id, \
         shows.name, \
         shows.description, \
         shows.url, \
         shows.image_url \
         FROM episodes \
         LEFT JOIN shows ON \
         episodes.show_id = shows.id \
         ORDER BY date_published DESC \
         LIMIT 10",
    )?;

    let mut episodes: Vec<Episode> = vec![];
    let mut rows = statement.query(params![])?;

    while let Some(row) = rows.next()? {
        episodes.push(Episode {
            id: row.get(0)?,
            title: row.get(1)?,
            description: match row.get::<usize, String>(2) {
                Ok(description) => Some(description)
                    .filter(|description| !description.is_empty()),
                _ => None,
            },
            url: row.get(3)?,
            guid: match row.get::<usize, String>(4) {
                Ok(guid) => Some(guid).filter(|guid| !guid.is_empty()),
                _ => None,
            },
            image_url: row.get(5)?,
            date_published: row.get(6)?,
            show: match row.get(7) {
                Ok(id) => Some(Show {
                    id,
                    name: row.get(8)?,
                    description: match row.get::<usize, String>(9) {
                        Ok(description) => Some(description)
                            .filter(|description| !description.is_empty()),
                        _ => None,
                    },
                    url: match row.get::<usize, String>(10) {
                        Ok(url) => Some(url).filter(|url| !url.is_empty()),
                        _ => None,
                    },
                    image_url: match row.get::<usize, String>(11) {
                        Ok(image) => {
                            Some(image).filter(|image| !image.is_empty())
                        }
                        _ => None,
                    },
                }),
                _ => None,
            },
        })
    }

    Ok(episodes)
}
