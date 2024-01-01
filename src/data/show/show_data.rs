/* show_data.rs
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

use rusqlite::{params, Transaction};

use crate::data::show::show_model::ShowModel;

pub struct ShowData;

impl ShowData {
    pub fn save_transaction(transaction: &Transaction, show: &ShowModel) {
        let mut statement = transaction
            .prepare(
                "REPLACE INTO shows (\
                 id, \
                 name, \
                 description, \
                 url, \
                 image_url \
                 ) VALUES (?,?,?,?,?)",
            )
            .expect("Failed to prepare query");

        statement
            .execute(params![
                show.id,
                show.name,
                show.description,
                show.url,
                show.image_url
            ])
            .expect("Failed to save show");
    }

    // pub fn load_episodes(
    //     connection: &PooledConnection<SqliteConnectionManager>,
    // ) -> Vec<EpisodeModel> {
    //     let mut statement = database.prepare(
    //         "SELECT \
    //          episodes.id, \
    //          episodes.title, \
    //          episodes.description, \
    //          episodes.url, \
    //          episodes.guid, \
    //          episodes.image_url, \
    //          episodes.date_published, \
    //          shows.id, \
    //          shows.name, \
    //          shows.description, \
    //          shows.url, \
    //          shows.image_url \
    //          FROM episodes \
    //          LEFT JOIN shows ON \
    //          episodes.show_id = shows.id \
    //          ORDER BY date_published DESC \
    //          LIMIT 12",
    //     )?;

    //     let mut episodes: Vec<Episode> = vec![];
    //     let mut rows = statement.query([])?;

    //     while let Some(row) = rows.next()? {
    //         episodes.push(Episode {
    //             id: row.get(0)?,
    //             title: row.get(1)?,
    //             description: match row.get::<usize, String>(2) {
    //                 Ok(description) => Some(description)
    //                     .filter(|description| !description.is_empty()),
    //                 _ => None,
    //             },
    //             url: row.get(3)?,
    //             guid: match row.get::<usize, String>(4) {
    //                 Ok(guid) => Some(guid).filter(|guid| !guid.is_empty()),
    //                 _ => None,
    //             },
    //             image_url: row.get(5)?,
    //             date_published: row.get(6)?,
    //             show: match row.get(7) {
    //                 Ok(id) => Some(Show {
    //                     id,
    //                     name: row.get(8)?,
    //                     description: match row.get::<usize, String>(9) {
    //                         Ok(description) => Some(description)
    //                             .filter(|description| !description.is_empty()),
    //                         _ => None,
    //                     },
    //                     url: match row.get::<usize, String>(10) {
    //                         Ok(url) => Some(url).filter(|url| !url.is_empty()),
    //                         _ => None,
    //                     },
    //                     image_url: match row.get::<usize, String>(11) {
    //                         Ok(image) => {
    //                             Some(image).filter(|image| !image.is_empty())
    //                         }
    //                         _ => None,
    //                     },
    //                 }),
    //                 _ => None,
    //             },
    //         });
    //     }

    //     Ok(episodes)
    // }
}
