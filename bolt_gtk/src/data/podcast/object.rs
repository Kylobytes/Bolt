/* object.rs
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

use std::cell::RefCell;

use adw::prelude::*;
use gtk::glib::{self, subclass::prelude::*, Properties};

use crate::{api::search::result::SearchResult, data::podcast::Podcast};

mod imp {
    use super::*;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::PodcastObject)]
    pub struct PodcastObject {
        #[property(name = "id", get, construct_only, type = i64, member = id)]
        #[property(name = "name", get, construct_only, type = Option<String>, member = name)]
        #[property(name = "description", get, construct_only, type = Option<String>, member = description)]
        #[property(name = "url", get, construct_only, type = Option<String>, member = url)]
        #[property(name = "image-url", get, construct_only, type = Option<String>, member = image_url)]
        data: RefCell<Podcast>,
        #[property(get, set)]
        subscribed: RefCell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PodcastObject {
        const NAME: &'static str = "PodcastObject";
        type Type = super::PodcastObject;
    }

    #[glib::derived_properties]
    impl ObjectImpl for PodcastObject {}
}

glib::wrapper! {
    pub struct PodcastObject(ObjectSubclass<imp::PodcastObject>);
}

impl Default for PodcastObject {
    fn default() -> Self {
        glib::Object::builder::<Self>().build()
    }
}

impl From<SearchResult> for PodcastObject {
    fn from(show: SearchResult) -> Self {
        glib::Object::builder::<Self>()
            .property("id", show.id)
            .property("name", Some(show.title))
            .property(
                "description",
                Some(show.description).filter(|text| !text.is_empty()),
            )
            .property("url", Some(show.url).filter(|url| !url.is_empty()))
            .property(
                "image-url",
                Some(show.image).filter(|image| !image.is_empty()),
            )
            .property("subscribed", false)
            .build()
    }
}

impl PodcastObject {
    pub fn mark_subscribed(&self) {
        self.set_property("subscribed", true);
    }
}
