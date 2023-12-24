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

use crate::data::database;
use crate::data::model::show::Show;

pub fn load_all_shows() -> Result<Vec<Show>, Box<dyn Error>> {
    let pool = database::connect();
    let connection = pool.get().expect("Failed to connect to database");

    let mut statement = connection.prepare(
        "SELECT \
         id, \
         name, \
         description, \
         url, \
         image_url \
         FROM shows \
         WHERE shows.id IN (SELECT subscriptions.show_id FROM subscriptions)",
    )?;

    let mut rows = statement.query([])?;
    let mut shows: Vec<Show> = vec![];

    while let Some(row) = rows.next()? {
        shows.push(Show {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            url: row.get(3)?,
            image_url: row.get(4)?,
        });
    }

    Ok(shows)
}
