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

use std::{io::Cursor, path::PathBuf};

use reqwest::header::CONTENT_TYPE;

use crate::api::CLIENT;

pub async fn fetch_image(url: &str) -> Result<Vec<u8>, reqwest::Error> {
    Ok(CLIENT.get(url).send().await?.bytes().await?.to_vec())
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
    let mut content = Cursor::new(&image_bytes);

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
