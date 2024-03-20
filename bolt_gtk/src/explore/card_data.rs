/* card_data.rs
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

use std::cell::{Cell, RefCell};

use gtk::glib::{self, prelude::*, subclass::prelude::*, Properties};

use crate::api::search::result::SearchResult;

mod imp {
    use super::*;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::CardData)]
    pub struct CardData {
        #[property(name = "id", get, construct_only)]
        pub id: Cell<i64>,
        #[property(get, construct_only)]
        pub name: RefCell<String>,
        #[property(get, construct_only)]
        pub description: RefCell<String>,
        #[property(get, construct_only)]
        pub image_url: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CardData {
        const NAME: &'static str = "CardData";
        type Type = super::CardData;
    }

    #[glib::derived_properties]
    impl ObjectImpl for CardData {}
}

glib::wrapper! {
    pub struct CardData(ObjectSubclass<imp::CardData>);
}

impl Default for CardData {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl From<SearchResult> for CardData {
    fn from(search_result: SearchResult) -> Self {
        glib::Object::builder::<Self>()
            .property("id", search_result.id)
            .property("name", search_result.title)
            .property("description", search_result.description)
            .property("image-url", search_result.image)
            .build()
    }
}
