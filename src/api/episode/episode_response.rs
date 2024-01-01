/* episode.rs
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

use std::error::Error;

use gtk::glib;
use rusqlite::{params, Transaction};
use serde::{Deserialize, Serialize};
use ureq::AgentBuilder;

use crate::config::GETTEXT_PACKAGE;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeResponse {
    pub id: i64,
    pub title: String,
    pub link: String,
    pub description: String,
    pub guid: String,
    pub date_published: i64,
    pub date_published_pretty: String,
    pub date_crawled: i64,
    pub enclosure_url: String,
    pub enclosure_type: String,
    pub enclosure_length: i64,
    pub explicit: u8,
    pub episode: Option<i64>,
    pub episode_type: Option<String>,
    pub season: Option<i64>,
    pub image: String,
    pub feed_itunes_id: Option<i64>,
    pub feed_image: String,
    pub feed_id: i64,
    pub feed_title: String,
    pub feed_language: String,
}

impl EpisodeResponse {
    pub fn save_episode_transaction(
        &self,
        transaction: &Transaction,
    ) -> Result<usize, Box<dyn Error>> {
        let mut statement = transaction.prepare(
            "REPLACE INTO episodes (\
             id, \
             title, \
             description, \
             url, \
             guid, \
             image_url, \
             date_published, \
             show_id\
             ) VALUES (?,?,?,?,?,?,?,?)",
        )?;

        Ok(statement.execute(params![
            self.id,
            self.title,
            Some(&self.description)
                .filter(|description| !description.is_empty()),
            self.enclosure_url,
            Some(&self.guid).filter(|guid| !guid.is_empty()),
            Some(&self.image).filter(|image| !image.is_empty()),
            self.date_published,
            self.feed_id,
        ])?)
    }

    pub fn save_show_transaction(
        &self,
        transaction: &Transaction,
    ) -> Result<usize, Box<dyn Error>> {
        let mut statement = transaction.prepare(
            "REPLACE INTO shows (\
             id, \
             name, \
             description, \
             url, \
             image_url \
             ) VALUES (?,?,?,?,?)",
        )?;

        Ok(statement.execute(params![
            self.feed_id,
            self.feed_title,
            None::<String>,
            self.link,
            Some(&self.feed_image).filter(|image| !image.is_empty())
        ])?)
    }

    pub fn save_image(&self) -> Result<(), Box<dyn Error>> {
        if self.feed_image.is_empty() && self.image.is_empty() {
            return Ok(());
        }

        let mut cache_dir = glib::user_cache_dir();
        cache_dir.push(GETTEXT_PACKAGE);
        cache_dir.push("tmp");
        cache_dir.push("images");

        if !cache_dir.as_path().exists() {
            let path_created = std::fs::create_dir_all(cache_dir.clone());

            match path_created {
                Ok(_) => println!("Created Bolt cache directory"),
                Err(message) => println!(
                    "Failed to create Bolt cache directory: {}",
                    message
                ),
            };
        }

        let mut path = cache_dir.clone();
        path.push(self.feed_id.to_string());

        if path.as_path().exists() {
            return Ok(());
        }

        let image_url: String = if self.feed_image.is_empty() {
            self.image.to_string()
        } else {
            self.feed_image.to_string()
        };

        let mut image = std::fs::File::create(path)?;
        let agent = AgentBuilder::new().build();
        let mut response = agent.get(&image_url).call()?.into_reader();
        let _ = std::io::copy(
            &mut response,
            &mut std::io::BufWriter::new(&mut image),
        )?;

        Ok(())
    }
}
