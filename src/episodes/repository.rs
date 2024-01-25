/* view.rs
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
 *https://api.podcastindex.org/api/1.0/recent/feeds?pretty
 * You should have received a copy of the GNU General Public License
 * along with Bolt. If not, see <https://www.gnu.org/licenses/>.
 *
 */

use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

use crate::data::{
    database,
    episode::{self, Episode},
};

pub fn load_episodes() -> Vec<Episode> {
    let database: PooledConnection<SqliteConnectionManager> =
        database::connect()
            .get()
            .expect("Failed to connect to database pool");

    let episodes = episode::model::load_episodes(&database);

    episodes
}
