/* show.rs
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
 *
 */

use std::cell::RefCell;

use adw::prelude::*;
use gtk::glib::{self, subclass::prelude::*, Properties};

use crate::api::search::result::SearchResult;

#[derive(Clone, Debug, Default)]
pub struct ShowData {
    pub id: i64,
    pub title: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
}

mod imp {
    use super::*;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::DiscoverShow)]
    pub struct DiscoverShow {
        #[property(name = "id", get, construct_only, type = i64, member = id)]
        #[property(name = "title", get, construct_only, type = Option<String>, member = title)]
        #[property(name = "description", get, construct_only, type = Option<String>, member = description)]
        #[property(name = "image", get, construct_only, type = Option<String>, member = image)]
        data: RefCell<ShowData>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DiscoverShow {
        const NAME: &'static str = "DiscoverShow";
        type Type = super::DiscoverShow;
    }

    #[glib::derived_properties]
    impl ObjectImpl for DiscoverShow {}
}

glib::wrapper! {
    pub struct DiscoverShow(ObjectSubclass<imp::DiscoverShow>);
}

impl Default for DiscoverShow {
    fn default() -> Self {
        glib::Object::builder::<Self>().build()
    }
}

impl From<SearchResult> for DiscoverShow {
    fn from(show: SearchResult) -> Self {
        glib::Object::builder::<Self>()
            .property("id", show.id)
            .property("title", Some(show.title))
            .property(
                "description",
                Some(show.description).filter(|text| !text.is_empty()),
            )
            .property(
                "image",
                Some(show.image).filter(|image| !image.is_empty()),
            )
            .build()
    }
}
