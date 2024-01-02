/* utils.rs
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
 *
 */

use std::path::PathBuf;

use gtk::glib;

use crate::config::GETTEXT_PACKAGE;

pub fn episode_image_path(filename: &str) -> PathBuf {
    let mut image_path = glib::user_cache_dir();
    image_path.push(GETTEXT_PACKAGE);
    image_path.push("images");
    image_path.push("episodes");
    image_path.push(filename);

    image_path
}

pub fn show_image_path(filename: &str) -> PathBuf {
    let mut image_path = glib::user_cache_dir();
    image_path.push(GETTEXT_PACKAGE);
    image_path.push("images");
    image_path.push("shows");
    image_path.push(filename);

    image_path
}
