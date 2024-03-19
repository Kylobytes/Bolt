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
use reqwest::header::CONTENT_TYPE;

use crate::{api::CLIENT, config::GETTEXT_PACKAGE};

pub fn podcast_path(id: &str) -> PathBuf {
    let mut path = glib::user_data_dir();
    path.push(GETTEXT_PACKAGE);
    path.push("podcasts");
    path.push(id);

    path
}

pub fn podcast_image_cache(id: &str) -> PathBuf {
    let mut path = glib::user_cache_dir();
    path.push(GETTEXT_PACKAGE);
    path.push("images");
    path.push("podcasts");
    path.push(id);

    path
}

pub fn podcast_image(id: &str) -> Option<PathBuf> {
    let directory = podcast_image_cache(id);

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

pub fn episode_path(id: &str, podcast_id: &str) -> PathBuf {
    let mut path = podcast_path(podcast_id);
    path.push("episodes");
    path.push(id);

    path
}

pub fn episode_image(id: &str, podcast_id: &str) -> Option<PathBuf> {
    let directory = episode_path(id, podcast_id);

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

pub async fn save_image(
    url: &str,
    directory: &PathBuf,
    filename: &str,
) -> Result<(), reqwest::Error> {
    let response = CLIENT.get(url).send().await?;
    let content_type = response.headers()[CONTENT_TYPE].to_str().unwrap();
    let extension = match content_type {
        "image/jpg" => "jpg",
        "image/jpeg" => "jpg",
        "image/png" => "png",
        _ => "png",
    };

    let image_bytes = response
        .bytes()
        .await
        .expect("Failed to download image bytes");

    if !directory.as_path().exists() {
        std::fs::create_dir_all(directory)
            .expect("Failed to create image directory");
    }

    let mut path: PathBuf =
        [directory.to_str().unwrap(), filename].iter().collect();
    path.set_extension(&extension);

    let mut image =
        std::fs::File::create(&path).expect("Failed to create image at path");
    let mut content = std::io::Cursor::new(&image_bytes);

    std::io::copy(&mut content, &mut std::io::BufWriter::new(&mut image))
        .expect("Failed to save image");

    Ok(())
}

pub async fn download_episode_media(url: &str, directory: &PathBuf) {
    let response = CLIENT
        .get(url)
        .send()
        .await
        .expect("Failed to download episode media");

    let filetype = response.headers()[CONTENT_TYPE].to_str().unwrap();
    let extension = match filetype {
        "audio/mpeg" => "mp3",
        "audio/ogg" => "ogg",
        _ => "mp3",
    };

    let episode_bytes = response
        .bytes()
        .await
        .expect("Failed to download episode_bytes");

    let mut path: PathBuf = [directory.to_str().unwrap()].iter().collect();

    if !path.exists() {
        std::fs::create_dir_all(&path).expect("Failed to create episode path");
    }

    path.push("media");
    path.set_extension(&extension);

    let mut media =
        std::fs::File::create(&path).expect("Failed to create episode file");
    let mut content = std::io::Cursor::new(&episode_bytes);

    std::io::copy(&mut content, &mut std::io::BufWriter::new(&mut media))
        .expect("Failed to save episode file");
}
