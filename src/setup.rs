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

use crate::data::database;
use crate::config::GETTEXT_PACKAGE;
use crate::config::PKGDATADIR;

pub fn run() {
    setup_data_dir();
    initialize_database();
}

fn setup_data_dir() {
    let mut user_dir = glib::user_data_dir();
    user_dir.push(GETTEXT_PACKAGE);

    if !user_dir.as_path().exists() {
        let path_created = std::fs::create_dir_all(user_dir);

        match path_created {
            Ok(_) => println!("Created Bolt data directory"),
            Err(message) => {
                println!("Failed to create data directory: {}", message)
            }
        };
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

        transaction.execute(&contents, []).expect("Failed to run migration");
    }

    let _ = transaction.commit();
}
