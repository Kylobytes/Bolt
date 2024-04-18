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

use std::{borrow::BorrowMut, path::PathBuf};

use gtk::glib;
use reqwest::{
    header::{HeaderValue, CONTENT_TYPE},
    Response,
};
use tokio::{fs::File, io::AsyncWriteExt};

use crate::config::GETTEXT_PACKAGE;

pub fn podcast_path(id: &str) -> PathBuf {
    let mut path = glib::user_data_dir();
    path.push(GETTEXT_PACKAGE);
    path.push("podcasts");
    path.push(id);

    path
}

pub fn podcast_image_path(id: &str) -> PathBuf {
    let mut path = glib::user_cache_dir();
    path.push(GETTEXT_PACKAGE);
    path.push("images");
    path.push("podcasts");
    path.push(id);

    path
}

pub fn podcast_image(id: &str) -> Option<PathBuf> {
    let directory = podcast_image_path(id);

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
) -> Result<(), anyhow::Error> {
    let mut response: Response = reqwest::get(url).await?;

    if !response.status().is_success() {
        anyhow::bail!("Url returned a non 200 status code");
    }

    let header_value: Option<&HeaderValue> =
        response.headers().get(CONTENT_TYPE);

    if header_value.is_none() {
        anyhow::bail!("Could not determine the url's content type");
    }

    let content_type: &HeaderValue = header_value.unwrap();
    let extension: String = match content_type.to_str()? {
        "image/png" => "png".to_string(),
        "image/jpg" => "jpg".to_string(),
        "image/jpeg" => "jpg".to_string(),
        _ => anyhow::bail!("Could not save unsupported format"),
    };

    if !directory.exists() {
        std::fs::create_dir_all(directory)?;
    }

    let mut path: PathBuf = directory.to_path_buf();
    path.push("cover");
    path.set_extension(&extension);

    let mut file: File = File::create(&path).await?;

    while let Some(mut content) = response.chunk().await? {
        file.write_all_buf(content.borrow_mut()).await?;
    }
    // let content: Bytes = response.bytes().await?;
    // std::io::copy(&mut content.as_ref(), &mut file)?;

    Ok(())
}

pub async fn download_enclosure(_url: &str, _directory: &PathBuf) {}
