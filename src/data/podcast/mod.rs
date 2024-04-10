/* mod.rs
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

use gtk::glib::{self, prelude::*, subclass::prelude::*, Properties};

use crate::api::search::result::SearchResult;

#[derive(Clone, Debug, Default)]
struct Data {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub url: Option<String>,
    pub image_url: Option<String>,
    pub subscribed: bool,
}

mod imp {
    use super::*;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::Podcast)]
    pub struct Podcast {
        #[property(name = "id", get, construct_only, type = i64, member = id)]
        #[property(name = "name", get, construct_only, type = String, member = name)]
        #[property(name = "description", get, construct_only, type = Option<String>, member = description)]
        #[property(name = "url", get, construct_only, type = Option<String>, member = url)]
        #[property(name = "image-url", get, construct_only, type = Option<String>, member = image_url)]
        #[property(name = "subscribed", get, set, type = bool, member = subscribed)]
        data: RefCell<Data>,
    }
    #[glib::object_subclass]
    impl ObjectSubclass for Podcast {
        const NAME: &'static str = "Podcast";
        type Type = super::Podcast;
    }

    #[glib::derived_properties]
    impl ObjectImpl for Podcast {}
}

glib::wrapper! {
    pub struct Podcast(ObjectSubclass<imp::Podcast>);
}

impl Default for Podcast {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl From<SearchResult> for Podcast {
    fn from(search_result: SearchResult) -> Self {
        glib::Object::builder::<Self>()
            .property("id", search_result.id)
            .property("name", Some(search_result.title))
            .property(
                "description",
                Some(search_result.description)
                    .filter(|text| !text.is_empty()),
            )
            .property(
                "url",
                Some(search_result.url).filter(|url| !url.is_empty()),
            )
            .property(
                "image-url",
                Some(search_result.image).filter(|image| !image.is_empty()),
            )
            .property("subscribed", false)
            .build()
    }
}

impl Podcast {
    pub fn mark_subscribed(&self) {
        self.set_property("subscribed", true);
    }
}
