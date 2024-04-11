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

use anyhow::anyhow;
use gtk::glib;
use sqlx::{query, Pool, Sqlite};

use crate::{
    config::{GETTEXT_PACKAGE, PKGDATADIR},
    data::database,
    runtime,
};

pub fn run() {
    setup_directories();

    runtime().block_on(async move {
        if let Err(message) = initialize_database().await {
            println!("Error initializing database: {message}");
        }
    });
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

async fn initialize_database() -> Result<(), anyhow::Error> {
    let pool: Pool<Sqlite> = database::connect().await?;

    query!("CREATE TABLE IF NOT EXISTS migrations (\
             id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, \
             name TEXT NOT NULL, \
             migrated_at INTEGER NOT NULL DEFAULT (UNIXEPOCH(CURRENT_TIMESTAMP)))")
        .execute(&pool)
        .await?;

    let migrations: String = format!("{}/migrations", PKGDATADIR);

    let read_dir = std::fs::read_dir(&migrations)?;

    for entry in read_dir.into_iter() {
        let migration = entry?.path().clone();
        let migration_name = migration
            .clone()
            .file_stem()
            .ok_or(anyhow!("Unable to acquire the filename"))?
            .to_str()
            .ok_or(anyhow!("Unable to parse the filename"))?
            .to_string();

        if let Ok(contents) = std::fs::read_to_string(&migration) {
            query(&contents).execute(&pool).await?;

            query("INSERT INTO migrations (name) VALUES (?)")
                .bind(&migration_name)
                .execute(&pool)
                .await?;
        }
    }

    Ok(())
}
