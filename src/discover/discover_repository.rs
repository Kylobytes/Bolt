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

use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Statement};

use crate::{
    api::{
        connection::ApiConnection, episode_client::EpisodeClient,
        recent_episodes::RecentEpisodes, AGENT,
    },
    data::{
        database,
        episode::{episode_data::EpisodeData, episode_model::EpisodeModel},
        show::{show_data::ShowData, show_model::ShowModel},
    },
};

pub struct DiscoverRepository;

impl DiscoverRepository {
    pub fn fetch_recent_episodes() -> Result<Vec<EpisodeModel>, Box<dyn Error>>
    {
        let api_connection: ApiConnection = ApiConnection::builder()
            .build_url("/recent/episodes?max=12")
            .build_authentication_headers()
            .build();

        let recent_episodes: RecentEpisodes =
            EpisodeClient::fetch_recent(&AGENT, &api_connection);
        let pool = database::connect();
        let mut database: PooledConnection<SqliteConnectionManager> =
            pool.get()?;

        let episodes: Vec<EpisodeModel> = recent_episodes
            .items
            .iter()
            .map(|episode| EpisodeModel::from(episode.clone()))
            .collect();

        let transaction = database.transaction()?;

        for episode in episodes.iter() {
            if let Some(show) = &episode.show {
                ShowData::save_transaction(&transaction, &show);
                EpisodeData::save_transaction(&transaction, &episode);
            }
        }

        transaction.commit()?;

        Ok(episodes)
    }

    pub fn find_episode_by_id(id: i64) -> Option<EpisodeModel> {
        let pool = database::connect();
        let database: PooledConnection<SqliteConnectionManager> =
            pool.get().expect("Failed to build connection pool");

        let mut statement: Statement = database
            .prepare(
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
             WHERE episodes.id = ? \
             LIMIT 1",
            )
            .expect("Failed to prepare query");

        let mut rows = statement
            .query_map(params![id], |row| {
                Ok(EpisodeModel {
                    id: row.get(0).unwrap(),
                    title: row.get(1).unwrap(),
                    description: match row.get::<usize, String>(2) {
                        Ok(description) => {
                            Some(description).filter(|description: &String| {
                                !description.is_empty()
                            })
                        }
                        _ => None,
                    },
                    url: row.get(3).unwrap(),
                    guid: match row.get::<usize, String>(4) {
                        Ok(guid) => {
                            Some(guid).filter(|guid: &String| !guid.is_empty())
                        }
                        _ => None,
                    },
                    image_url: row.get(5).unwrap(),
                    date_published: row.get(6).unwrap(),
                    show: match row.get(7) {
                        Ok(id) => Some(ShowModel {
                            id,
                            name: row.get(8).unwrap(),
                            description: match row.get::<usize, String>(9) {
                                Ok(description) => Some(description).filter(
                                    |description: &String| {
                                        !description.is_empty()
                                    },
                                ),
                                _ => None,
                            },
                            url: match row.get::<usize, String>(10) {
                                Ok(url) => Some(url)
                                    .filter(|url: &String| !url.is_empty()),
                                _ => None,
                            },
                            image_url: match row.get::<usize, String>(11) {
                                Ok(image) => {
                                    Some(image).filter(|image: &String| {
                                        !image.is_empty()
                                    })
                                }
                                _ => None,
                            },
                        }),
                        _ => None,
                    },
                })
            })
            .expect("Failed to find episode");

        let Some(result) = rows.next() else {
            return None;
        };

        let Ok(episode) = result else {
            return None;
        };

        Some(episode)
    }
}
