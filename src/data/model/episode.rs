/* episode.rs
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

use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Rows, Statement};

use crate::data::{
    self, model::show::Show, remote::episode::Episode as RemoteEpisode,
};

#[derive(Clone, Debug)]
pub struct Episode {
    pub id: i64, // podcastindex.org's id
    pub title: String,
    pub description: Option<String>,
    pub url: String,
    pub guid: Option<String>,
    pub image_url: Option<String>,
    pub date_published: i64,
    pub show: Option<Show>,
}

impl From<RemoteEpisode> for Episode {
    fn from(episode: RemoteEpisode) -> Self {
        Episode {
            id: episode.id,
            title: episode.title,
            description: Some(episode.description)
                .filter(|description| !description.is_empty()),
            url: episode.link,
            guid: Some(episode.guid).filter(|guid| !guid.is_empty()),
            image_url: Some(episode.image).filter(|image| !image.is_empty()),
            date_published: episode.date_published,
            show: Some(Show {
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

impl Episode {
    pub fn find_by_id(id: i64) -> Self {
        let pool: Pool<SqliteConnectionManager> = data::database::connect();
        let database: PooledConnection<SqliteConnectionManager> =
            pool.get().expect("Failed to load database");
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
            .unwrap();

        let mut rows: Rows<'_> = statement
            .query(params![id])
            .expect("Failed to fetch episodes");
        let mut episodes: Vec<Episode> = vec![];

        while let Some(row) =
            rows.next().expect("Failed to iterate through episodes")
        {
            episodes.push(Episode {
                id: row.get(0).unwrap(),
                title: row.get(1).unwrap(),
                description: match row.get::<usize, String>(2) {
                    Ok(description) => Some(description)
                        .filter(|description| !description.is_empty()),
                    _ => None,
                },
                url: row.get(3).unwrap(),
                guid: match row.get::<usize, String>(4) {
                    Ok(guid) => Some(guid).filter(|guid| !guid.is_empty()),
                    _ => None,
                },
                image_url: row.get(5).unwrap(),
                date_published: row.get(6).unwrap(),
                show: match row.get(7) {
                    Ok(id) => Some(Show {
                        id,
                        name: row.get(8).unwrap(),
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
            });
        }

        episodes.first().unwrap().clone()
    }
}
