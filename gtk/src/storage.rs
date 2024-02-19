/* storage.rs
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

use std::path::PathBuf;

use gtk::glib;

use crate::config::GETTEXT_PACKAGE;

pub fn show_path(id: &str) -> PathBuf {
    let mut path = glib::user_data_dir();
    path.push(GETTEXT_PACKAGE);
    path.push("shows");
    path.push(id);

    path
}

pub fn show_image(id: &str) -> Option<PathBuf> {
    let directory = show_path(id);

    let Ok(contents) = std::fs::read_dir(&directory) else {
        return None;
    };

    contents
        .filter_map(|file| file.ok())
        .filter(|file| file.path().is_file())
        .filter(|file| {
            let Ok(filename) = file.file_name().into_string() else {
                return false;
            };

            return filename == "cover.png" || filename == "cover.jpg";
        })
        .map(|file| file.path())
        .collect::<Vec<PathBuf>>()
        .first()
        .cloned()
}

pub fn episode_path(id: &str, show_id: &str) -> PathBuf {
    let mut path = show_path(show_id);
    path.push("episodes");
    path.push(id);

    path
}

pub fn episode_image(id: &str, show_id: &str) -> Option<PathBuf> {
    let directory = episode_path(id, show_id);

    let Ok(contents) = std::fs::read_dir(&directory) else {
        return None;
    };

    contents
        .filter_map(|file| file.ok())
        .filter(|file| file.path().is_file())
        .filter(|file| {
            let Ok(filename) = file.file_name().into_string() else {
                return false;
            };

            return filename == "cover.png" || filename == "cover.jpg";
        })
        .map(|file| file.path())
        .collect::<Vec<PathBuf>>()
        .first()
        .cloned()
}
