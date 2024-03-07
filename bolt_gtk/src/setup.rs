/* setup.rs
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

use bolt_migration::{Migrator, MigratorTrait};
use sea_orm::Database;

use crate::{config::GETTEXT_PACKAGE, runtime};

pub fn run() {
    setup_directories();
    runtime().block_on(initialize_database());
}

fn setup_directories() {
    let mut user_dir = glib::user_data_dir();
    user_dir.push(GETTEXT_PACKAGE);

    if !user_dir.as_path().exists() {
        let path_created = std::fs::create_dir_all(user_dir);

        match path_created {
            Ok(_) => println!("Created Bolt data directory"),
            Err(message) => {
                println!("Failed to create Bolt data directory: {message}")
            }
        };
    }

    let mut show_image_directory = glib::user_data_dir();
    show_image_directory.push(GETTEXT_PACKAGE);
    show_image_directory.push("shows");

    if !show_image_directory.as_path().exists() {
        let path_created = std::fs::create_dir_all(&show_image_directory);

        match path_created {
            Ok(_) => println!("Created Bolt show image directory"),
            Err(message) => {
                println!(
                    "Failed to create Bolt show image directory: {message}"
                );
            }
        }
    }
}

async fn initialize_database() {
    let path = format!(
        "sqlite://{}/{}/{}.db?mode=rwc",
        glib::user_data_dir().display(),
        GETTEXT_PACKAGE,
        GETTEXT_PACKAGE
    );

    let connection = Database::connect(path)
        .await
        .expect("Failed to connect to database");

    Migrator::up(&connection, None)
        .await
        .expect("Failed to run migrations");
}
