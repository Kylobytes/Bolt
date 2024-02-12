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

pub fn show_image_path(id: &str) -> PathBuf {
    let mut path = show_path(id);
    path.push("cover");

    path
}

pub fn episode_path(id: &str, show_id: &str) -> PathBuf {
    let mut path = show_path(show_id);
    path.push("episodes");
    path.push(id);

    path
}

pub fn episode_image_path(id: &str, show_id: &str) -> PathBuf {
    let mut path = episode_path(id, show_id);
    path.push("episodes");
    path.push(id);
    path.push("image");

    path
}

pub fn episode_file_path(id: &str, show_id: &str, filename: &str) -> PathBuf {
    let mut path = episode_path(id, show_id);
    path.push("episodes");
    path.push(id);
    path.push(filename);

    path
}
