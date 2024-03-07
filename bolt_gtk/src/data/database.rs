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
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio::sync::OnceCell;

use crate::config::GETTEXT_PACKAGE;

pub async fn connect() -> &'static DatabaseConnection {
    static CONNECTION: OnceCell<DatabaseConnection> = OnceCell::const_new();

    let path = format!(
        "sqlite://{}/{}/{}.db?mode=rwc",
        glib::user_data_dir().display(),
        GETTEXT_PACKAGE,
        GETTEXT_PACKAGE
    );

    CONNECTION
        .get_or_init(|| async {
            let mut options = ConnectOptions::new(&path);
            options.max_connections(5);

            Database::connect(options)
                .await
                .expect("Failed to connect to database")
        })
        .await
}
