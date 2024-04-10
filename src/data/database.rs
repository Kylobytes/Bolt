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

use gtk::glib;
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::{Error, Pool};

use crate::config::GETTEXT_PACKAGE;

pub fn connect() -> Result<Pool<SqliteConnectionManager>, Error> {
    let path = format!(
        "sqlite://{}/{}/{}.db?mode=rwc",
        glib::user_data_dir().display(),
        GETTEXT_PACKAGE,
        GETTEXT_PACKAGE
    );

    let manager = SqliteConnectionManager::file(&path);

    Pool::new(manager)
}
