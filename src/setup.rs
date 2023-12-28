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

use std::path::PathBuf;

use gtk::glib;

use crate::config::GETTEXT_PACKAGE;
use crate::config::PKGDATADIR;
use crate::data::database;

pub fn run() {
    setup_directories();
    initialize_database();
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

    let mut episode_image_cache = glib::user_cache_dir();
    episode_image_cache.push(GETTEXT_PACKAGE);
    episode_image_cache.push("images");
    episode_image_cache.push("episodes");

    if !episode_image_cache.as_path().exists() {
        let path_created = std::fs::create_dir_all(episode_image_cache);

        match path_created {
            Ok(_) => println!("Created Bolt episode image cache"),
            Err(message) => {
                println!(
                    "Failed to create Bolt episode image cache: {message}"
                );
            }
        }
    }

    let mut show_image_cache = glib::user_cache_dir();
    show_image_cache.push(GETTEXT_PACKAGE);
    show_image_cache.push("images");
    show_image_cache.push("shows");

    if !show_image_cache.as_path().exists() {
        let path_created = std::fs::create_dir_all(show_image_cache);

        match path_created {
            Ok(_) => println!("Created Bolt show image cache"),
            Err(message) => {
                println!("Failed to create Bolt show image cache: {message}");
            }
        }
    }
}

fn initialize_database() {
    let pool = database::connect();
    let mut connection = pool.get().expect("Could not connect to database");
    let transaction = connection
        .transaction()
        .expect("Failed to start transaction for migrations");

    let path: PathBuf = [PKGDATADIR, "migrations"].iter().collect();
    let files = std::fs::read_dir(path).expect("Failed to acquire migrations");

    for file in files {
        let migration_path = file.unwrap().path();
        let contents = std::fs::read_to_string(migration_path).unwrap();

        transaction
            .execute(&contents, [])
            .expect("Failed to run migration");
    }

    let _ = transaction.commit();
}
