/* database.rs
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

use once_cell::sync::Lazy;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use crate::config::GETTEXT_PACKAGE;
use gtk::glib;

static POOL: Lazy<Pool<SqliteConnectionManager>> = Lazy::new(|| {
    let mut database_url = glib::user_data_dir();
    database_url.push(GETTEXT_PACKAGE);
    database_url.push(GETTEXT_PACKAGE);
    database_url.set_extension("db");

    let manager = SqliteConnectionManager::file(database_url);
    Pool::new(manager).expect("Unable to initialize database pool")
});

pub fn connect() -> Pool<SqliteConnectionManager> {
    POOL.clone()
}
