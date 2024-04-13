/* mod.rs
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

use std::cell::RefCell;

use adw::prelude::*;
use gtk::glib::{self, subclass::prelude::*, Properties};

use crate::api::episode::Episode as ApiEpisode;

#[derive(Default, Debug)]
struct Data {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub url: String,
    pub image_url: Option<String>,
    pub enclosure_url: String,
    pub date_published: i64,
    pub queued: bool,
    pub played: bool,
    pub podcast_id: i64,
}

mod imp {
    use super::*;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::Episode)]
    pub struct Episode {
        #[property(name = "id", get, construct_only, type = i64, member = id)]
        #[property(name = "title", get, construct_only, type = String, member = title)]
        #[property(name = "description", get, construct_only, type = Option<String>, member = description)]
        #[property(name = "url", get, construct_only, type = String, member = url)]
        #[property(name = "image-url", get, construct_only, type = Option<String>, member = image_url)]
        #[property(name = "enclosure-url", get, construct_only, type = String, member = enclosure_url)]
        #[property(name = "queued", get, construct_only, type = bool, member = queued)]
        #[property(name = "played", get, construct_only, type = bool, member = played)]
        #[property(name = "date-published", get, construct_only, type = i64, member = date_published)]
        #[property(name = "podcast-id", get, construct_only, type = i64, member = podcast_id)]
        data: RefCell<Data>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Episode {
        const NAME: &'static str = "Episode";
        type Type = super::Episode;
    }

    #[glib::derived_properties]
    impl ObjectImpl for Episode {}
}

glib::wrapper! {
    pub struct Episode(ObjectSubclass<imp::Episode>);
}

impl Default for Episode {
    fn default() -> Self {
        glib::Object::new::<Self>()
    }
}

impl From<ApiEpisode> for Episode {
    fn from(episode: ApiEpisode) -> Self {
        glib::Object::builder::<Self>()
            .property("id", episode.id)
            .property("title", episode.title)
            .property(
                "description",
                Some(episode.description).filter(|text| !text.is_empty()),
            )
            .property("url", episode.link)
            .property(
                "image-url",
                Some(episode.image).filter(|image| !image.is_empty()),
            )
            .property("enclosure-url", episode.enclosure_url)
            .property("queued", false)
            .property("date-published", episode.date_published)
            .property("podcast-id", episode.feed_id)
            .build()
    }
}
